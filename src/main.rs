#![feature(test)]
#![feature(box_into_inner)]

mod hm;
mod child;
mod pval;
mod bucket;
mod pbst;

extern crate test;

use std::collections::HashMap;
use num::Zero;
use num_bigint::BigUint;

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
    let b256 = BigUint::from(256 as u16);
    let mut r = vec![0;len].into_boxed_slice();
    for i in 0..len {
        let idk = (int.clone()%b256.clone()).to_bytes_be();
        r[len-i-1] = idk[0];
        int = int.clone() >> 8;
    }
    r.to_vec()
}

/*struct No{
    key: BigUint,
    value: BigUint,
    next: No
}*/


#[cfg(test)]
mod tests{
    use std::collections::{BTreeSet, HashSet};
    use super::*;
    use test::Bencher;
    use num::One;
    use crate::hm::HashMap;

    #[test]
    fn hmt(){
        // BigUint::from();
        let x = HashMap::new();
        // x.put("alo", 1);
    }

    #[bench]
    fn x(b: &mut Bencher){
        let mut s = BTreeSet::new();
        b.iter(|| {
            s.insert("SapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapan");
            // println!("{}",s.contains("alo"));
            // println!("{}",s.contains("alox"));
            s.insert("SapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapan");
            // s.get("alo");
            for i in &s{

            }
            // println!("{}",s.get("alo").unwrap());
        });

        /*let m = BigUint::from(16 as usize);
        let k = btoi(b"SapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapan".to_vec());
        println!("{}",k);
        b.iter(|| {
            let h = k.modpow(&BigUint::one(), &m);
        });*/
    }

    #[bench]
    fn foo(b: &mut Bencher){
        let x = b"SapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapanSapan";
        b.iter(|| {
            // println!("{:?}", x);
            let mut i = btoi(x.to_vec());
            // println!("{}",i);
            let s = itob(&mut i);
            // println!("{:?}", s);
        });
    }
}

fn main() {

}