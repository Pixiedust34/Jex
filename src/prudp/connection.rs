use crate::NexResult;

pub trait Endpoint: Send + Sync {
	fn send(&self, data: &[u8]) -> NexResult<()>;
	fn receive(&self) -> NexResult<Vec<u8>>;
}

pub trait Connection: Send + Sync {
	fn connect(&mut self) -> NexResult<()>;
	fn disconnect(&mut self) -> NexResult<()>;
}


