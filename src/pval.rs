use std::cmp::Ordering;
use num::{ToPrimitive, Zero};
use num_bigint::BigUint;

#[derive(Default, Debug, Clone, Eq, Hash, PartialOrd)]
pub struct PVal{
    val: BigUint,
    lt: usize
}

impl Ord for PVal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialEq for PVal {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl PVal{
    pub fn put(val: BigUint) -> Self{
        PVal{ val: val.clone(), lt: (val%BigUint::from(100 as u64)).to_usize().unwrap() }
    }
    pub fn get(&self) -> BigUint{
        self.val.clone()
    }

    pub fn lt(&self) -> usize {
        self.lt.clone()
    }

    fn btoi(bytes: Vec<u8>) -> BigUint{
        let mut r = BigUint::zero();
        for byte in bytes {
            r = (r << 8) | BigUint::from(byte);
        }
        r
    }

    fn itob(int: &mut BigUint) -> Vec<u8>{
        let mut int = int.clone();
        let len = ((int.to_bytes_be().len()<<3) + 7) >> 3;
        let b256 = BigUint::from(256 as u64);
        let mut r = vec![0;len].into_boxed_slice();
        for i in 0..len {
            let idk = (int.clone()%b256.clone()).to_bytes_be();
            r[len-i-1] = idk[0];
            int = int.clone() >> 8;
        }
        r.to_vec()
    }

}