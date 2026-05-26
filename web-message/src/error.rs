#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("missing '{0}' field")]
	MissingField(&'static str),

	#[error("invalid '{0}' field: {1}")]
	InvalidField(&'static str, String),

	#[error("unexpected length")]
	UnexpectedLength,

	#[error("unexpected type")]
	UnexpectedType,

	#[error("unknown tag")]
	UnknownTag,

	#[cfg(feature = "Url")]
	#[error("invalid URL: {0}")]
	InvalidUrl(url::ParseError),
}
