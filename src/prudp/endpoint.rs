use crate::NexResult;

#[derive(Debug, Default)]
pub struct PrudpEndpoint;

impl PrudpEndpoint {
	pub fn new() -> Self { Self }
	pub fn send(&self, _data: &[u8]) -> NexResult<()> { Ok(()) }
	pub fn receive(&self) -> NexResult<Vec<u8>> { Ok(Vec::new()) }
}


