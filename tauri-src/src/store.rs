use shared::traits::{Creatable, Entity, Updatable};
use shared::traits::Queryable;
use std::collections::{BTreeMap, HashMap};
use surrealdb::{Datastore, Session};
use surrealdb::sql::*;
use tokio::sync::Mutex;
use crate::error::Error;
use crate::MouseLoc;
use crate::prelude::*;
use crate::utils::*;

pub struct Store {
    pub datastore: Datastore,
    pub session: Session,
    pub mouse_loc: Mutex<HashMap<u32, MouseLoc>>,
}

impl Store {
    pub async fn new() -> Result<Self> {
        let datastore = Datastore::new("file://test.db").await?;
        let session = Session::for_db("appns", "appdb");
        Ok(Store {
            datastore,
            session,
            mouse_loc: Mutex::new(HashMap::new()),
        })
    }

    pub async fn exec_get<T>(
        &self,
        tid: Option<String>,
        nested_fields: Option<&str>,
    ) -> Result<Vec<Object>>
        where
            T: Entity<DatabaseType=Object> + Queryable,
    {
        let sql = match nested_fields {
            Some(x) => {
                if tid.is_some() {
                    format!("SELECT *, {} FROM $th", x)
                } else {
                    format!("SELECT *, {} FROM {}", x, T::table_name())
                }
            }
            None => {
                if tid.is_some() {
                    format!("SELECT * FROM $th")
                } else {
                    format!("SELECT * FROM {}", T::table_name())
                }
            }
        };
        let vars = match tid {
            Some(ref tid) => {
                map!["th".into() => Thing::from((T::table_name(), tid.clone())).into()]
            }
            None => BTreeMap::new(),
        };
        let ress = self
            .datastore
            .execute(sql.as_str(), &self.session, Some(vars), true)
            .await?;
        let first_res = ress.into_iter().next().expect("Did not get a response");
        let res: Vec<Value> = first_res.result?.try_into()?;
        let res = res
            .into_iter()
            .map(|f| Object::try_from(f))
            .filter_map(|f| f.ok())
            .collect();
        Ok(res)
    }

    pub async fn exec_create<T>(&self, data: T) -> Result<String>
        where
            T: Creatable + Entity<DatabaseType=Object>,
    {
        let sql = "CREATE type::table($tb) CONTENT $data RETURN id";
        let tb = <T as Entity>::table_name();
        let mut data: Object = data.into().into();
        let now = Datetime::default().timestamp_nanos();
        data.insert("ctime".into(), now.into());

        let vars = map![
            "tb".into() => <T as Entity>::table_name().into(),
            "data".into() => Value::from(data)];

        let ress = self
            .datastore
            .execute(sql, &self.session, Some(vars), false)
            .await?;
        let first_val = ress
            .into_iter()
            .next()
            .map(|r| r.result)
            .expect("id not returned")?;

        if let Value::Object(mut val) = first_val.first() {
            match val.remove("id").ok_or(Error::XPropertyNotFound(format!(
                "exec_create {tb} id not found"
            )))? {
                Value::Thing(x) => Ok(x.id.to_string()),
                _ => Err(Error::StoreFailToCreate(format!("exec_create {tb}"))),
            }
        } else {
            Err(Error::StoreFailToCreate(format!(
                "exec_create {tb}, nothing returned."
            )))
        }
    }

    pub async fn exec_delete(&self, tid: &str) -> Result<String> {
        let sql = "DELETE $th";

        let vars = map!["th".into() => thing(tid)?.into()];

        let ress = self
            .datastore
            .execute(sql, &self.session, Some(vars), false)
            .await?;

        let first_res = ress.into_iter().next().expect("Did not get a response");

        // Return the error if result failed
        first_res.result?;

        // return success
        Ok(tid.to_string())
    }

    pub async fn exec_select(&self, tb: &str, filter: Option<Value>) -> Result<Vec<Object>> {
        let mut sql = String::from("SELECT * FROM type::table($tb)");

        let mut vars = BTreeMap::from([("tb".into(), tb.into())]);

        // --- Apply the filter
        if let Some(filter) = filter {
            let obj: Object = filter.try_into()?;
            sql.push_str(" WHERE");
            for (idx, (k, v)) in obj.into_iter().enumerate() {
                let var = format!("w{idx}");
                sql.push_str(&format!(" {k} = ${var}"));
                vars.insert(var, v);
            }
        }

        // --- Apply the orderby
        sql.push_str(" ORDER ctime DESC");

        let ress = self
            .datastore
            .execute(&sql, &self.session, Some(vars), false)
            .await?;

        let first_res = ress.into_iter().next().expect("Did not get a response");

        // Get the result value as value array (fail if it is not)
        let array: Vec<Value> = first_res.result?.try_into()?;
        let mut v: Vec<Object> = Vec::new();
        for i in array.into_iter() {
            v.push(i.try_into()?);
        }
        Ok(v)
    }

    pub async fn exec_update<T: Updatable + Entity<DatabaseType = Object>>(
        &self,
        tb: String,
        data: T,
        filter: Option<Object>,
    ) -> Result<()> {
        let mut sql = String::from("UPDATE $tb MERGE $va");

        let mut vars = BTreeMap::from([
            ("tb".into(), tb.into()),
            ("va".into(), Value::Object(data.into())),
        ]);

        // --- Apply the filter
        if let Some(filter) = filter {
            sql.push_str(" WHERE");
            for (idx, (k, v)) in filter.into_iter().enumerate() {
                let var = format!("w{idx}");
                sql.push_str(&format!(" {k} = ${var}"));
                vars.insert(var, v);
            }
        }
        let _ = self
            .datastore
            .execute(&sql, &self.session, Some(vars), false)
            .await?;

        Ok(())
    }
}

impl XTakeImpl<String> for Object {
    fn x_take_impl(&mut self, k: &str) -> Result<Option<String>> {
        let v = self.remove(k).map(|v| v.try_into());
        match v {
            None => Ok(None),
            Some(Ok(val)) => Ok(Some(val)),
            Some(Err(ex)) => Err(ex.into()),
        }
    }
}

impl XTakeImpl<i64> for Object {
    fn x_take_impl(&mut self, k: &str) -> Result<Option<i64>> {
        let v = self.remove(k).map(|v| v.try_into());
        match v {
            None => Ok(None),
            Some(Ok(val)) => Ok(Some(val)),
            Some(Err(ex)) => Err(ex.into()),
        }
    }
}

impl XTakeImpl<bool> for Object {
    fn x_take_impl(&mut self, k: &str) -> Result<Option<bool>> {
        Ok(self.remove(k).map(|v| v.is_true()))
    }
}
