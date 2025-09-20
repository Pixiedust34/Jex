#[derive(Debug, Default)]
pub struct TimeoutManager;

impl TimeoutManager {
	pub fn new() -> Self { Self::default() }
	pub fn tick(&mut self) {}
}


