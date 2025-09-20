use crate::NexResult;

pub trait Readable: Sized {
	fn read_from<R: std::io::Read>(reader: &mut R) -> NexResult<Self>;
}

pub trait Writable {
	fn write_to<W: std::io::Write>(&self, writer: &mut W) -> NexResult<()>;
}

pub mod pid;
pub mod station_url;