use crate::constants::{PrudpPacketType, PrudpPacketFlags};

#[derive(Debug, Clone)]
pub struct PrudpHeader {
	pub version: u8,
	pub source: u8,
	pub destination: u8,
	pub type_id: PrudpPacketType,
	pub flags: PrudpPacketFlags,
	pub session_id: u8,
	pub signature: u32,
}

#[derive(Debug, Clone)]
pub struct PrudpPacket {
	pub header: PrudpHeader,
	pub payload: Vec<u8>,
}


