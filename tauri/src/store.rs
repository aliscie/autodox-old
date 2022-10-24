use crate::error::Error;
use crate::prelude::*;
use crate::utils::*;
use crate::MouseLoc;
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

        W(first_res.result?.first()).try_into()
    }

    pub async fn exec_create<T: Into<Value>>(&self, tb: &str, data: T) -> Result<String> {
        let sql = "CREATE type::table($tb) CONTENT $data RETURN id";

        let mut data: Object = W(data.into()).try_into()?;
        let now = Datetime::default().timestamp_nanos();
        data.insert("ctime".into(), now.into());

        let vars = map![
            "tb".into() => tb.into(),
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
            val.x_take_val::<String>("id")
                .map_err(|ex| Error::StoreFailToCreate(format!("exec_create {tb} {ex}")))
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
            let obj: Object = W(filter).try_into()?;
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
        let array: Array = W(first_res.result?).try_into()?;

        // build the list of objects
        array.into_iter().map(|value| W(value).try_into()).collect()

        // Note: Above equivalent to
        // let mut objs: Vec<Object> = Vec::new();
        // for item in array.into_iter() {
        // 	objs.push(W(item).try_into()?);
        // }
        // Ok(objs)
    }
}



impl XTakeImpl<String> for Object {
	fn x_take_impl(&mut self, k: &str) -> Result<Option<String>> {
		let v = self.remove(k).map(|v| W(v).try_into());
		match v {
			None => Ok(None),
			Some(Ok(val)) => Ok(Some(val)),
			Some(Err(ex)) => Err(ex),
		}
	}
}

impl XTakeImpl<i64> for Object {
	fn x_take_impl(&mut self, k: &str) -> Result<Option<i64>> {
		let v = self.remove(k).map(|v| W(v).try_into());
		match v {
			None => Ok(None),
			Some(Ok(val)) => Ok(Some(val)),
			Some(Err(ex)) => Err(ex),
		}
	}
}

impl XTakeImpl<bool> for Object {
	fn x_take_impl(&mut self, k: &str) -> Result<Option<bool>> {
		Ok(self.remove(k).map(|v| v.is_true()))
	}
}
