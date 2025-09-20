use crate::io::{ByteStreamIn, ByteStreamOut};
use crate::NexResult;

#[derive(Debug, Clone, Default)]
pub struct RmcMessage {
	pub is_request: bool,
	pub is_success: bool,
	pub is_hpp: bool,
	pub protocol_id: u16,
	pub call_id: u32,
	pub method_id: u32,
	pub error_code: u32,
	pub parameters: Vec<u8>,
}

impl RmcMessage {
	pub fn from_bytes_packed(mut data: Vec<u8>) -> NexResult<Self> {
		let mut s = ByteStreamIn::new(data, None, None);
		let _length = s.read_u32_le()?;
		let protocol = s.read_u8()?;
		let mut msg = RmcMessage::default();
		msg.protocol_id = (protocol & !0x80) as u16;
		if msg.protocol_id == 0x7F { msg.protocol_id = s.read_u16_le()?; }
		if protocol & 0x80 != 0 {
			msg.is_request = true;
			msg.call_id = s.read_u32_le()?;
			msg.method_id = s.read_u32_le()?;
			msg.parameters = s.read_remaining();
		} else {
			msg.is_request = false;
			msg.is_success = s.read_bool()?;
			if msg.is_success {
				msg.call_id = s.read_u32_le()?;
				msg.method_id = s.read_u32_le()? & !0x8000;
				msg.parameters = s.read_remaining();
			} else {
				msg.error_code = s.read_u32_le()?;
				msg.call_id = s.read_u32_le()?;
			}
		}
		Ok(msg)
	}

	pub fn to_bytes_packed(&self) -> Vec<u8> {
		let mut inner = ByteStreamOut::new(None, None);
		let flag = if self.is_request { 0x80u16 } else { 0 };
		if !self.is_hpp || (self.is_hpp && self.is_request) {
			if self.protocol_id < 0x80 { inner.write_u8((self.protocol_id as u8) | (flag as u8)); }
			else { inner.write_u8(0x7F | (flag as u8)); inner.write_u16_le(self.protocol_id); }
		}
		if self.is_request {
			inner.write_u32_le(self.call_id);
			inner.write_u32_le(self.method_id);
			inner.write(&self.parameters);
		} else {
			inner.write_bool(self.is_success);
			if self.is_success {
				inner.write_u32_le(self.call_id);
				inner.write_u32_le(self.method_id | 0x8000);
				inner.write(&self.parameters);
			} else {
				inner.write_u32_le(self.error_code);
				inner.write_u32_le(self.call_id);
			}
		}
		let mut msg = ByteStreamOut::new(None, None);
		msg.write_u32_le(inner.bytes().len() as u32);
		msg.write(inner.bytes());
		msg.bytes().to_vec()
	}
}


