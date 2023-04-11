use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use num::ToPrimitive;
use num_bigint::BigUint;
use crate::hashes_test::Hashes;
use crate::pval::PVal;

pub struct AS{
    bkts: Vec<HashSet<KeyPair>>
}

#[derive(Default,Clone,Debug,Eq,Hash,PartialEq)]
pub struct KeyPair {
    key: PVal,
    val: PVal
}



impl KeyPair {
    pub fn new(key: PVal, val: PVal) -> Self{
        KeyPair{ key, val }
    }
}
impl AS {
    pub fn new() -> Self{
        let mut bkts = Vec::new();
        for _ in 0..256 {
            bkts.push(HashSet::default());
        }
        Self{
            bkts
        }
    }
    pub fn insert(&mut self, pair: KeyPair) -> bool {
        let x = &mut self.bkts[Self::get_hash(pair.clone())];
        x.insert(pair);
        true
    }
    fn get_hash(pair: KeyPair) -> usize {
        let size = BigUint::from(256 as u64);
        let hash = Hashes::default();
        let fnva1 = hash.fnva1;
        fnva1.hash(pair.key.clone(),size).get().to_usize().unwrap()
    }
   /* pub fn get(&mut self, key: PVal) -> PVal{

    }*/
}