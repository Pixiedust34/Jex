use crate::NexResult;
use crate::io::{ByteStreamIn, ByteStreamOut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pid(pub u64);

impl Pid {
	pub fn write_to(&self, out: &mut ByteStreamOut) {
		if out.pid_size() == 8 { out.write_u64_le(self.0); } else { out.write_u32_le(self.0 as u32); }
	}

	pub fn read_from(input: &mut ByteStreamIn) -> NexResult<Self> {
		if input.pid_size() == 8 { Ok(Pid(input.read_u64_le()?)) } else { Ok(Pid(input.read_u32_le()? as u64)) }
	}
}


