use num::One;
use num_bigint::BigUint;
use crate::pval::PVal;

#[derive(Default)]
pub struct Hashes{
    pub fnva1: FNVa1
}

#[derive(Default)]
pub struct FNVa1;

impl FNVa1{
    pub fn hash(&self, key: PVal, size: BigUint) -> PVal{
        let mut hash = BigUint::parse_bytes(b"cbf29ce484222325", 16).unwrap() % size.clone();
        let prime = BigUint::parse_bytes(b"100000001b3", 16).unwrap();
        let key = key.get().to_bytes_le();
        for i in key {
            hash = (hash ^ BigUint::from(i)).modpow(&BigUint::one(), & size);
            // hash = (hash * prime.clone()) % size.clone();
        }
        PVal::put(hash)
    }
}