use std::usize::MAX;
use num::One;
use num_bigint::BigUint;
use crate::pval::PVal;

#[derive(Default)]
pub struct Hashes{
    pub fnva1: FNVa1
}

#[derive(Default)]
pub struct FNVa1{
    prime: usize,
    hash: usize
}

impl FNVa1{
    pub fn init() -> Self{
        // let hash = BigUint::parse_bytes(b"cbf29ce484222325", 16).unwrap();
        // let prime = BigUint::parse_bytes(b"100000001b3", 16).unwrap();
        let hash = 0x811c9dc5;
        let prime = 0x01000193;
        Self{
            prime,
            hash
        }
    }
    pub fn hash(&self, key: &[u8]) -> usize{
        let mut hash = self.hash;
        for i in key {
            hash = (hash ^ &(*i as usize));
            hash = ((hash) * &self.prime);
        }
        hash
    }
}