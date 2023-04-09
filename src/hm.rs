use std::any::TypeId;
use std::collections::BTreeSet;
use std::fmt::Display;
use num::{ToPrimitive, Zero};
use num_bigint::BigUint;
use crate::child::Child;
use crate::pval::PVal;

pub struct HashMap {
    list: Vec<&'static mut Child>,
}

impl HashMap {
    pub fn new() {
        /*let mut v = Vec::new();
        for _ in 0..256 {
            v.push(&mut Child::default());
        }
        HashMap {
            list: v
        }*/
    }

    pub fn put(&self, key: PVal, val: PVal){
        // put as last char
       /* let e = self.list[key.lt()];
        let c = Child {
            key,
            val,
            ptr: None,
            occupied: 1,
        };
        e.insert(c);*/
    }
}