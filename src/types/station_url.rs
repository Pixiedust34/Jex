use crate::constants::{StationUrlFlag, StationUrlType};
use crate::io::{ByteStreamIn, ByteStreamOut};
use crate::NexResult;

#[derive(Debug, Clone, Default)]
pub struct StationUrl {
	url_type: StationUrlType,
	url: String,
	flags: u8,
}

impl StationUrl {
	pub fn new(url: String) -> Self {
		let mut s = Self { url_type: StationUrlType::Unknown, url, flags: 0 };
		s.parse();
		s
	}

	pub fn write_to(&self, out: &mut ByteStreamOut) { self.string(out); }

	pub fn extract_from(input: &mut ByteStreamIn) -> NexResult<Self> {
		let url = Self::read_string(input)?;
		let mut s = Self::new(url);
		s.parse();
		Ok(s)
	}

	pub fn url(&self) -> String { let mut s = self.clone(); s.format(); s.url }

	pub fn is_public(&self) -> bool { self.flags & StationUrlFlag::PUBLIC.bits() != 0 }
	pub fn is_behind_nat(&self) -> bool { self.flags & StationUrlFlag::BEHIND_NAT.bits() != 0 }

	fn parse(&mut self) {
		let url = self.url.clone();
		if url.is_empty() || url.len() > 1024 { return; }
		if let Some((scheme, _rest)) = url.split_once(":/") {
			self.url_type = match scheme {
				"prudp" => StationUrlType::Prudp,
				"prudps" => StationUrlType::Prudps,
				"udp" => StationUrlType::Udp,
				_ => StationUrlType::Unknown,
			};
		}
	}

	fn format(&mut self) {
		self.url = match self.url_type {
			StationUrlType::Prudp => "prudp:/".to_string(),
			StationUrlType::Prudps => "prudps:/".to_string(),
			StationUrlType::Udp => "udp:/".to_string(),
			StationUrlType::Unknown => String::new(),
		};
	}

	fn string(&self, out: &mut ByteStreamOut) { let s = self.url(); Self::write_string(out, &s); }

	fn write_string(out: &mut ByteStreamOut, s: &str) {
		let bytes = s.as_bytes();
		if out.string_length_size() == 4 { out.write_u32_le(bytes.len() as u32); } else { out.write_u16_le(bytes.len() as u16); }
		out.write(bytes);
	}

	fn read_string(input: &mut ByteStreamIn) -> NexResult<String> {
		let len = if input.string_length_size() == 4 { input.read_u32_le()? as usize } else { input.read_u16_le()? as usize };
		let bytes = input.read(len as u64)?;
		Ok(String::from_utf8_lossy(&bytes).into_owned())
	}
}


