pub use uuid_set::*;
pub use x_take::*;

mod uuid_set;
mod x_take;
// from: https://github.com/surrealdb/surrealdb.wasm/blob/main/src/mac/mod.rs
macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
		let mut m = ::std::collections::BTreeMap::new();
        $(m.insert($k, $v);)+
        m
    }};
  }
pub(crate) use map; // export macro for crate
