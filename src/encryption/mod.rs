use crate::NexResult;
use crate::error::NexError;

pub trait Cipher: Send + Sync {
	fn encrypt(&self, data: &[u8]) -> NexResult<Vec<u8>>;
	fn decrypt(&self, data: &[u8]) -> NexResult<Vec<u8>>;
}

pub struct Dummy;

impl Cipher for Dummy {
	fn encrypt(&self, data: &[u8]) -> NexResult<Vec<u8>> { Ok(data.to_vec()) }
	fn decrypt(&self, data: &[u8]) -> NexResult<Vec<u8>> { Ok(data.to_vec()) }
}

pub struct Rc4 { key: Vec<u8> }

impl Rc4 {
	pub fn new(key: Vec<u8>) -> Self { Self { key } }
}

impl Cipher for Rc4 {
	fn encrypt(&self, data: &[u8]) -> NexResult<Vec<u8>> {
		Ok(rc4(self.key.as_slice(), data))
	}

	fn decrypt(&self, data: &[u8]) -> NexResult<Vec<u8>> {
		Ok(rc4(self.key.as_slice(), data))
	}
}

fn rc4(key: &[u8], data: &[u8]) -> Vec<u8> {
	let mut s: [u8; 256] = [0; 256];
	for i in 0..256 { s[i] = i as u8; }
	let mut j: u8 = 0;
	for i in 0..256u16 {
		let idx = i as usize;
		j = j.wrapping_add(s[idx]).wrapping_add(key[idx % key.len()]);
		s.swap(idx, j as usize);
	}
	let mut i: u8 = 0;
	let mut j2: u8 = 0;
	let mut out = Vec::with_capacity(data.len());
	for &b in data {
		i = i.wrapping_add(1);
		j2 = j2.wrapping_add(s[i as usize]);
		s.swap(i as usize, j2 as usize);
		let k = s[(s[i as usize].wrapping_add(s[j2 as usize])) as usize];
		out.push(b ^ k);
	}
	out
}


