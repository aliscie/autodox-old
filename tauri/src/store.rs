use crate::error::Error;
use crate::prelude::*;
use crate::utils::*;
use crate::MouseLoc;
use shared::traits::{Creatable, Entity};
use std::collections::{BTreeMap, HashMap};
use surrealdb::sql::*;
use surrealdb::{Datastore, Session};
use tokio::sync::Mutex;

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

    pub async fn exec_get(&self, tid: &str) -> Result<Object> {
        let sql = "SELECT * FROM $th";

        let vars = map!["th".into() => thing(tid)?.into()];

        let ress = self
            .datastore
            .execute(sql, &self.session, Some(vars), true)
            .await?;

        let first_res = ress.into_iter().next().expect("Did not get a response");
        Ok(first_res.result?.first().try_into()?)
    }

    pub async fn get_all<T: Entity<DatabaseType = Object>>(&self) -> Result<Object> {
        let sql = format!("SELECT * FROM {}", <T as Entity>::table_name());
        let ress = self
            .datastore
            .execute(sql.as_str(), &self.session, None, true)
            .await?;
        let first_res = ress.into_iter().next().expect("Did not get a response");
        println!("{:?}", first_res);
        Ok(first_res.result?.first().try_into()?)
    }

    pub async fn exec_create<T>(&self, data: T) -> Result<String>
    where
        T: Creatable + Entity<DatabaseType = Object>,
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
