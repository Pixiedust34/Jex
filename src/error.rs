use thiserror::Error;

#[derive(Debug, Error)]
pub enum NexError {
	#[error("IO error: {0}")]
	Io(#[from] std::io::Error),
	#[error("Parse error: {0}")]
	Parse(String),
	#[error("Unsupported: {0}")]
	Unsupported(String),
}
