#[derive(thiserror::Error, Debug)]
pub enum Error{
	#[error("Value not of type '{0}'")]
	XValueNotOfType(&'static str),

	#[error("Property '{0}' not found")]
	XPropertyNotFound(String),
}
