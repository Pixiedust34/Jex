use crate::NexResult;

#[derive(Debug, Default)]
pub struct SocketConnection;

impl SocketConnection {
	pub fn new() -> Self { Self }
	pub fn send(&self, _buf: &[u8]) -> NexResult<()> { Ok(()) }
	pub fn receive(&self) -> NexResult<Vec<u8>> { Ok(Vec::new()) }
}


