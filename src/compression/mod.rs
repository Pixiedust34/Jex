use crate::NexResult;
use crate::error::NexError;

pub trait CompressionAlgorithm: Send + Sync {
	fn compress(&self, input: &[u8]) -> NexResult<Vec<u8>>;
	fn decompress(&self, input: &[u8]) -> NexResult<Vec<u8>>;
	fn boxed_clone(&self) -> Box<dyn CompressionAlgorithm>;
}

pub struct Dummy;

impl CompressionAlgorithm for Dummy {
	fn compress(&self, input: &[u8]) -> NexResult<Vec<u8>> { Ok(input.to_vec()) }
	fn decompress(&self, input: &[u8]) -> NexResult<Vec<u8>> { Ok(input.to_vec()) }
	fn boxed_clone(&self) -> Box<dyn CompressionAlgorithm> { Box::new(Dummy) }
}

pub fn new_dummy() -> Dummy { Dummy }

pub struct Zlib;

impl CompressionAlgorithm for Zlib {
	fn compress(&self, input: &[u8]) -> NexResult<Vec<u8>> {
		use flate2::write::ZlibEncoder;
		use flate2::Compression;
		use std::io::Write;
		let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
		enc.write_all(input)?;
		let compressed = enc.finish()?;
		let ratio = if compressed.is_empty() { 0u8 } else { (input.len() / compressed.len() + 1) as u8 };
		let mut out = Vec::with_capacity(compressed.len() + 1);
		out.push(ratio);
		out.extend_from_slice(&compressed);
		Ok(out)
	}

	fn decompress(&self, input: &[u8]) -> NexResult<Vec<u8>> {
		if input.is_empty() { return Ok(Vec::new()); }
		let ratio = input[0] as usize;
		let compressed = &input[1..];
		if ratio == 0 { return Ok(compressed.to_vec()); }
		use flate2::read::ZlibDecoder;
		use std::io::Read;
		let mut dec = ZlibDecoder::new(compressed);
		let mut out = Vec::new();
		dec.read_to_end(&mut out)?;
		let ratio_check = if compressed.is_empty() { 0 } else { out.len() / compressed.len() + 1 };
		if ratio_check != ratio { return Err(NexError::Parse(format!("Bad zlib ratio. expected {ratio}, got {ratio_check}"))); }
		Ok(out)
	}

	fn boxed_clone(&self) -> Box<dyn CompressionAlgorithm> { Box::new(Zlib) }
}

pub fn new_zlib() -> Zlib { Zlib }

pub struct Lzo;

impl CompressionAlgorithm for Lzo {
	fn compress(&self, _input: &[u8]) -> NexResult<Vec<u8>> { Err(NexError::Unsupported("LZO compression not implemented".into())) }
	fn decompress(&self, _input: &[u8]) -> NexResult<Vec<u8>> { Err(NexError::Unsupported("LZO decompression not implemented".into())) }
	fn boxed_clone(&self) -> Box<dyn CompressionAlgorithm> { Box::new(Lzo) }
}

pub fn new_lzo() -> Lzo { Lzo }


