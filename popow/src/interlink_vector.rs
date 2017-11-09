extern crate primitives;

use std::io;
use chain::hash::H256;
use primitives::compact::Compact;
use primitives::bigint::U256;
use chain::BlockHeader;
use script::Script;
use script::Builder;
use ser::{Serializable, Deserializable, Error, Stream, Reader};
use merkle::merkle;


#[derive(Clone)]
#[derive(Serialize)]
#[derive(Debug)]
pub struct InterlinkVector {
	pub hash: H256,
	pub vector: Vec<H256>
}


impl InterlinkVector {

	pub fn update_with_header (&self, header: BlockHeader) -> InterlinkVector {
		let h = header.hash();
		let bits = header.bits;

		let level = max_level(bits, &h) as usize;
		if level == 0 {
			InterlinkVector{
				hash: h,
				vector: self.vector.clone()
			}
		} else {
			let mut new_vector = vec![h.clone(); level];
			if self.vector.len() - 1 > level {
				let (prefix, suffix) = self.vector.split_at(level);
				new_vector.extend(suffix.iter().cloned());
				InterlinkVector{
					hash: h,
					vector: new_vector
				}
			} else {
				let (genesis_hash, prefix) = self.vector.split_last().unwrap();
				new_vector.push(genesis_hash.clone());
				InterlinkVector{
					hash: h,
					vector: new_vector
				}
			}
		}
	}

	pub fn root(&self) -> H256 {
		merkle(&self.vector)
	}

	pub fn script(&self) -> Script {
		let vector_root = &*self.root() as &[u8];
		Builder::build_nulldata(vector_root)
	}
}

impl Serializable for InterlinkVector {
	fn serialize(&self, s: &mut Stream) {
		unimplemented!()
	}
}

impl Deserializable for InterlinkVector {
	fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
		unimplemented!()
	}
}




/// Returns true if hash is lower or equal than target and target is lower or equal
/// than current network maximum
pub fn is_on_level(bits: Compact, level: u8, hash: &H256) -> bool {
	let target = match bits.to_u256() {
		Ok(target) => target,
		_err => return false,
	};

	let value = U256::from(&*hash.reversed() as &[u8]);
	let better_target = target >> (level as usize) ;
	value <= better_target
}

pub fn max_level(bits: Compact, hash: &H256) -> u8 {
	let mut level = 0;
	while is_on_level(bits, level + 1, hash) {
		level = level + 1;
	}
	level
}


#[cfg(test)]
mod tests {

	extern crate test_data;

	use chain::IndexedBlock;
	use chain::hash::H256;
	use super::{InterlinkVector, is_on_level, max_level};


	#[test]
	fn test_block_header_stream() {
		assert_eq!(1,1);
	}

	#[test]
	fn test_is_on_level() {
		//block 1
		assert!(is_on_level(486604799u32.into(), 0,
							&H256::from_reversed_str("00000000839a8e6886ab5951d76f411475428afc90947ee320161bbf18eb6048")));

		// block 2
		assert!(is_on_level(486604799u32.into(), 1,
							&H256::from_reversed_str("000000006a625f06636b8bb6ac7b960a8d03705d1ace08b1a19da3fdcc99ddbd")));

		assert!(max_level(486604799u32.into(),
						  &H256::from_reversed_str("000000006a625f06636b8bb6ac7b960a8d03705d1ace08b1a19da3fdcc99ddbd")) == 1);

		// block 400_000
		assert!(is_on_level(403093919u32.into(), 0,
							&H256::from_reversed_str("000000000000000004ec466ce4732fe6f1ed1cddc2ed4b328fff5224276e3f6f")));

		// block 400_000
		assert!(is_on_level(486604799u32.into(), 37,
							&H256::from_reversed_str("000000000000000004ec466ce4732fe6f1ed1cddc2ed4b328fff5224276e3f6f")));

		// block 400_000
		assert!(!is_on_level(486604799u32.into(), 38,
							 &H256::from_reversed_str("000000000000000004ec466ce4732fe6f1ed1cddc2ed4b328fff5224276e3f6f")));

		// block 400_000
		assert!(max_level(486604799u32.into(),
						  &H256::from_reversed_str("000000000000000004ec466ce4732fe6f1ed1cddc2ed4b328fff5224276e3f6f")) == 37);


	}

	#[test]
	fn test_update() {
		let b0: IndexedBlock = test_data::block_h0().into();
		let b1: IndexedBlock = test_data::block_h1().into();
		let b2: IndexedBlock = test_data::block_h2().into();

		let genesis_hash = b0.header.hash.clone();

		let int_vec = InterlinkVector {
			hash: genesis_hash.clone(),
			vector: vec![genesis_hash]
		};

		let int_vec_1 = int_vec.update_with_header(b2.header.raw.clone());

		assert!(int_vec_1.vector.len() == 2);
		assert_eq!(int_vec_1.vector.first().unwrap().clone(), b2.header.raw.hash());
		assert_eq!(int_vec_1.vector.last().unwrap().clone(), b0.header.hash);

		let int_vec_2 = int_vec_1.update_with_header(b1.header.raw.clone());

		assert!(int_vec_2.vector.len() == 2);
		assert_eq!(int_vec_2.vector.first().unwrap().clone(), b2.header.raw.hash());
		assert_eq!(int_vec_2.vector.last().unwrap().clone(), b0.header.hash);


		let int_vec_3 = int_vec.update_with_header(b1.header.raw.clone());

		assert!(int_vec_3.vector.len() == 1);
		assert_eq!(int_vec_3.vector.first().unwrap().clone(), b0.header.hash);
	}
}
