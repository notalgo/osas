use std::any::TypeId;
use std::collections::BTreeSet;
use std::fmt::Display;
use avl::{AvlTreeMap, AvlTreeSet};
use num::{ToPrimitive, Zero};
use num_bigint::BigUint;
use crate::idk::{FooBST};
use crate::pval::PVal;

#[derive(Debug)]
pub struct HashMapx {
    list: Vec<AvlTreeMap<PVal, PVal>>,
    x: BigUint
}

impl HashMapx {
    pub fn new() -> HashMapx {
        let mut list = Vec::new();
        for _ in 0..256 {
            list.push(AvlTreeMap::new());
        }
        HashMapx {
            list,
            x: BigUint::from(0xf as u64),
        }
    }
    pub fn get(&mut self, key: PVal) -> Option<PVal>{
        // let x = (key.get()&self.x.clone()).to_usize().unwrap();
        match self.list[0].get(&key) {
            None => {
                None
            }
            Some(x) => {
                Some(x.clone())
            }
        }
    }
    pub fn insert(&mut self, key: PVal, val: PVal){
        // let x = (key.get()&self.x.clone()).to_usize().unwrap();
        self.list[0].insert(key, val);
    }
}
