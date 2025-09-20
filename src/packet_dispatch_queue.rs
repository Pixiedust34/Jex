#[derive(Debug, Default)]
pub struct PacketDispatchQueue;

impl PacketDispatchQueue {
	pub fn new() -> Self { Self::default() }
	pub fn enqueue(&mut self, _packet: Vec<u8>) {}
	pub fn drain(&mut self) {}
}


