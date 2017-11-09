use chain::hash::{H256, H512};
use crypto::sha256;


// Calculate closest bigger number of form 2^n, where n is unsigned int
#[inline]
fn bigger2n(k: u32) -> u32 {
	let lz = k.leading_zeros();
	let tz = k.trailing_zeros();
	match lz + tz {
		31 => k,
		_ => 2_u32.pow(32 - lz)
	}
}


//todo: domain separation
fn root(prev_level: &Vec<H256>) -> H256 {

	#[inline]
	fn concat<T>(a: T, b: T) -> H512 where T: AsRef<H256> {
		let mut result = H512::default();
		result[0..32].copy_from_slice(&**a.as_ref());
		result[32..64].copy_from_slice(&**b.as_ref());
		result
	}


	#[inline]
	fn two_elems_hash(left: H256, right: H256) -> H256 {
		sha256(&*concat(left, right))
	}

	let prev_length = prev_level.len();
	assert_eq!(prev_length % 2, 0);

	match prev_level.len() {
		2 => two_elems_hash(prev_level[0].clone(), prev_level[1].clone()),
		_ => {
			let level_size = prev_length / 2;
			let mut level: Vec<H256> = Vec::with_capacity(level_size);

			for idx in 0..level_size {
				let ed = two_elems_hash(prev_level[idx * 2].clone(), prev_level[idx * 2 + 1].clone());
				level.push(ed);
			};

			assert_eq!(level.len(), prev_length / 2);
			//let l = level.iter().map(|e| e.as_slice()).collect();
			root(&level)
		}
	}
}

pub fn merkle(non_empty_leafs: &Vec<H256>) -> H256 {
	let digest_size = 32;
	let zero_hash_vec = H256::default();
	let zero_hash = zero_hash_vec;

	let ne_count = non_empty_leafs.len() as u32;
	let l_count = bigger2n(ne_count);

	let mut leafs: Vec<H256> = Vec::with_capacity(l_count as usize);
	let neleafs: &Vec<H256> = non_empty_leafs;
	leafs.clone_from(neleafs);

	if l_count > ne_count {
		let mut eleafs: Vec<H256> = vec![zero_hash; (l_count - ne_count) as usize];
		leafs.append(&mut eleafs)
	}

	root(&leafs)
}


#[test]
fn test_merkle() {
	assert!(false);
}
