#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Fail to get Ctx")]
	CtxFail,

	#[error(transparent)]
	Surreal(#[from] surrealdb::Error),

	#[error(transparent)]
	IO(#[from] std::io::Error),
}
