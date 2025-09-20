use crate::error::NexError;
use crate::NexResult;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read};

#[derive(Debug, Clone, Default)]
pub struct ByteStreamSettings {
	pub string_length_size: usize, // 2 or 4
	pub pid_size: usize,           // 4 or 8
	pub use_structure_header: bool,
}

#[derive(Debug, Clone, Default)]
pub struct LibraryVersions;

#[derive(Debug, Clone)]
pub struct ByteStreamIn {
	data: Vec<u8>,
	pos: usize,
	pub library_versions: Option<LibraryVersions>,
	pub settings: Option<ByteStreamSettings>,
}

impl ByteStreamIn {
	pub fn new(data: Vec<u8>, library_versions: Option<LibraryVersions>, settings: Option<ByteStreamSettings>) -> Self {
		Self { data, pos: 0, library_versions, settings }
	}

	pub fn string_length_size(&self) -> usize {
		self.settings.as_ref().map(|s| s.string_length_size).unwrap_or(2)
	}

	pub fn pid_size(&self) -> usize {
		self.settings.as_ref().map(|s| s.pid_size).unwrap_or(4)
	}

	pub fn use_structure_header(&self) -> bool {
		self.settings.as_ref().map(|s| s.use_structure_header).unwrap_or(false)
	}

	pub fn remaining(&self) -> usize {
		self.data.len().saturating_sub(self.pos)
	}

	pub fn read_remaining(&mut self) -> Vec<u8> {
		let len = self.remaining();
		self.read(len as u64).unwrap_or_default()
	}

	pub fn read(&mut self, length: u64) -> NexResult<Vec<u8>> {
		let length = length as usize;
		if self.remaining() < length { return Err(NexError::Parse("Read is OOB".into())); }
		let start = self.pos;
		let end = self.pos + length;
		self.pos = end;
		Ok(self.data[start..end].to_vec())
	}

	fn cursor(&self) -> Cursor<&[u8]> {
		Cursor::new(&self.data[self.pos..])
	}

	pub fn read_u8(&mut self) -> NexResult<u8> {
		if self.remaining() < 1 { return Err(NexError::Parse("Not enough data to read u8".into())); }
		let v = self.data[self.pos];
		self.pos += 1;
		Ok(v)
	}

	pub fn read_u16_le(&mut self) -> NexResult<u16> {
		if self.remaining() < 2 { return Err(NexError::Parse("Not enough data to read u16".into())); }
		let mut cur = self.cursor();
		let v = cur.read_u16::<LittleEndian>()?;
		self.pos += 2;
		Ok(v)
	}

	pub fn read_u32_le(&mut self) -> NexResult<u32> {
		if self.remaining() < 4 { return Err(NexError::Parse("Not enough data to read u32".into())); }
		let mut cur = self.cursor();
		let v = cur.read_u32::<LittleEndian>()?;
		self.pos += 4;
		Ok(v)
	}

	pub fn read_u64_le(&mut self) -> NexResult<u64> {
		if self.remaining() < 8 { return Err(NexError::Parse("Not enough data to read u64".into())); }
		let mut cur = self.cursor();
		let v = cur.read_u64::<LittleEndian>()?;
		self.pos += 8;
		Ok(v)
	}

	pub fn read_i8(&mut self) -> NexResult<i8> { self.read_u8().map(|v| v as i8) }

	pub fn read_i16_le(&mut self) -> NexResult<i16> { self.read_u16_le().map(|v| v as i16) }

	pub fn read_i32_le(&mut self) -> NexResult<i32> { self.read_u32_le().map(|v| v as i32) }

	pub fn read_i64_le(&mut self) -> NexResult<i64> { self.read_u64_le().map(|v| v as i64) }

	pub fn read_f32_le(&mut self) -> NexResult<f32> {
		if self.remaining() < 4 { return Err(NexError::Parse("Not enough data to read f32".into())); }
		let mut cur = self.cursor();
		let v = cur.read_f32::<LittleEndian>()?;
		self.pos += 4;
		Ok(v)
	}

	pub fn read_f64_le(&mut self) -> NexResult<f64> {
		if self.remaining() < 8 { return Err(NexError::Parse("Not enough data to read f64".into())); }
		let mut cur = self.cursor();
		let v = cur.read_f64::<LittleEndian>()?;
		self.pos += 8;
		Ok(v)
	}

	pub fn read_bool(&mut self) -> NexResult<bool> {
		Ok(self.read_u8()? == 1)
	}
}

#[derive(Debug, Clone, Default)]
pub struct ByteStreamOut {
	buf: Vec<u8>,
	pub library_versions: Option<LibraryVersions>,
	pub settings: Option<ByteStreamSettings>,
}

impl ByteStreamOut {
	pub fn new(library_versions: Option<LibraryVersions>, settings: Option<ByteStreamSettings>) -> Self {
		Self { buf: Vec::new(), library_versions, settings }
	}

	pub fn string_length_size(&self) -> usize {
		self.settings.as_ref().map(|s| s.string_length_size).unwrap_or(2)
	}

	pub fn pid_size(&self) -> usize {
		self.settings.as_ref().map(|s| s.pid_size).unwrap_or(4)
	}

	pub fn use_structure_header(&self) -> bool {
		self.settings.as_ref().map(|s| s.use_structure_header).unwrap_or(false)
	}

	pub fn copy_new(&self) -> Self {
		Self { buf: Vec::new(), library_versions: self.library_versions.clone(), settings: self.settings.clone() }
	}

	pub fn write(&mut self, data: &[u8]) { self.buf.extend_from_slice(data); }

	pub fn write_u8(&mut self, v: u8) { self.buf.push(v); }

	pub fn write_u16_le(&mut self, v: u16) { self.buf.write_u16::<LittleEndian>(v).unwrap(); }

	pub fn write_u32_le(&mut self, v: u32) { self.buf.write_u32::<LittleEndian>(v).unwrap(); }

	pub fn write_u64_le(&mut self, v: u64) { self.buf.write_u64::<LittleEndian>(v).unwrap(); }

	pub fn write_i8(&mut self, v: i8) { self.buf.push(v as u8); }

	pub fn write_i16_le(&mut self, v: i16) { self.buf.write_i16::<LittleEndian>(v).unwrap(); }

	pub fn write_i32_le(&mut self, v: i32) { self.buf.write_i32::<LittleEndian>(v).unwrap(); }

	pub fn write_i64_le(&mut self, v: i64) { self.buf.write_i64::<LittleEndian>(v).unwrap(); }

	pub fn write_f32_le(&mut self, v: f32) { self.buf.write_f32::<LittleEndian>(v).unwrap(); }

	pub fn write_f64_le(&mut self, v: f64) { self.buf.write_f64::<LittleEndian>(v).unwrap(); }

	pub fn write_bool(&mut self, v: bool) { self.buf.push(if v {1} else {0}); }

	pub fn bytes(&self) -> &[u8] { &self.buf }
}
