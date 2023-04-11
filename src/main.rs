#![feature(test)]
#![feature(box_into_inner)]

mod hm;
mod pval;
mod pbst;
mod idk;
mod r#as;
mod hashes_test;
mod hm_try_4;

extern crate test;

use std::collections::HashMap;
use num::{One, Zero};
use num_bigint::{BigUint, RandBigInt};
use crate::hm::HashMapx;
use crate::hm_try_4::{HashMap4, KeyPair1};
use crate::pval::PVal;
use crate::r#as::{AS, KeyPair};

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
    use num_bigint::RandBigInt;
    use rand::Rng;
    use crate::hm::HashMapx;
    use crate::idk::FooBST;
    use crate::pval::PVal;
    use crate::r#as::{AS, KeyPair};

    #[test]
    fn lin(){
        let mut x= AS::new();
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let k = rng.gen_biguint(64);
            let v = rng.gen_biguint(64);
            let k = PVal::put(k);
            let v = PVal::put(v);
            x.insert(KeyPair::new(k, v));
        }
    }

    #[test]
    fn foo(){
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            v.push(rng.gen_biguint(64));
        }
        let mut tree = FooBST::new();
        for x in v.iter() {
            let mut pv = PVal::put(x.to_owned());
            tree.insert(pv.clone(), pv);
        }
        println!("{:#?}", tree);
    }

    #[test]
    fn hmt(){
        // BigUint::from();
        let mut x = HashMapx::new();
        let ten = PVal::put(BigUint::from(10 as u64));
        x.insert(ten.clone(), PVal::put(BigUint::one()));
        println!("{}", x.get(ten).unwrap().get());

    }

    #[bench]
    fn foox(b: &mut Bencher){
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

#[cfg(test)]
mod fb{
    use test::Bencher;

    #[bench]
    fn fb_4(b:&mut Bencher){
        let r = 1000;
        b.iter(||{
            for i in 1..=r {

            }
        });
    }

    #[bench]
    fn fb_3(b: &mut Bencher){
        let r = 1000;
        b.iter(||{
            let mut x = 1;
            let mut y = 1;
            let mut o;
            for i in 1..=r {
                o = true;
                if x==3 {
                    // assert!(x%15!=0);
                    assert!(i%3==0);
                    // assert!(x%5!=0);
                    x = 0;
                    o = false;
                }
                if y==5 {
                    // assert!(x%15!=0);
                    // assert!(x%3!=0);
                    assert!(i%5==0);
                    // print!("Buzz");
                    y = 0;
                    o = false;
                }
                if o {
                    assert!(i%15!=0);
                    assert!(i%3!=0);
                    assert!(i%5!=0);
                }
                x += 1;y += 1;
                // println!();
            }
        });
    }

    #[bench]
    fn fb_2(b: &mut Bencher){
        let r = 1000;
        b.iter(||{
            let mut o;
            for x in 1..=r{
                o = true;
                if x%3 == 0 {
                    assert!(x%3==0);
                    o = false;
                }
                if x%5 == 0 {
                    // print!("Buzz");
                    assert!(x%5==0);
                    o = false;
                }
                if o {
                    assert!(x%15!=0);
                    assert!(x%3!=0);
                    assert!(x%5!=0);
                }
                // println!();
            }
        });
    }

    #[bench]
    fn fb_1(b: &mut Bencher){
        let r = 1000;
        b.iter(||{
            for x in 1..=r{
                if x%15 == 0 {
                    assert!(x%15==0);
                }else if x%3 == 0 {
                    assert!(x%3==0);
                } else if x%5 == 0 {
                    assert!(x%5==0);
                }else {
                    assert!(x%15!=0);
                    assert!(x%3!=0);
                    assert!(x%5!=0);
                }
             }
        });
    }
}

#[cfg(test)]
mod bench_comp {
    use std::collections::{BTreeMap, BTreeSet, HashMap};
    use test::Bencher;
    use avl::AvlTreeMap;
    use num_bigint::RandBigInt;
    use crate::hm::HashMapx;
    use crate::idk::FooBST;
    use crate::pval::PVal;
    use crate::r#as::{AS, KeyPair};

    static SIZE: u64 = 10000;

    #[bench]
    fn a(b: &mut Bencher){
        let mut x= AS::new();
        let mut rng = rand::thread_rng();
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..SIZE {
            v.push(rng.gen_biguint(64));
        }
        b.iter(||{
            for i in v.iter() {
                let k = i.clone();
                // let v = i.clone();
                let k = PVal::put(k);
                // let v = PVal::put(v);
                x.insert(KeyPair::new(k.clone(), k));
            }
        });
    }

    #[bench]
    fn x(b: &mut Bencher){
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..SIZE {
            v.push(rng.gen_biguint(64));
        }
        let mut tree = FooBST::new();
        for x in v.iter() {
            let mut pv = PVal::put(x.to_owned());
            tree.insert(pv.clone(), pv);
        }
        // println!("{:#?}", tree);
        b.iter(||{
            for x in v.iter() {
                tree.get(PVal::put(x.clone()));
            }
        });
    }

    #[bench]
    fn y(b: &mut Bencher){
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..SIZE {
            v.push(rng.gen_biguint(64));
        }
        let mut hm = HashMap::new();
        for x in v.iter() {
            let mut pv = PVal::put(x.to_owned());
            hm.insert(pv.clone(), pv);
        }
        b.iter(||{
            for x in v.iter() {
                hm.get(&PVal::put(x.clone()));
            }
        });
    }
    #[bench]
    fn z(b: &mut Bencher){
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..SIZE {
            v.push(rng.gen_biguint(64));
        }
        let mut hm = AvlTreeMap::new();
        for x in v.iter() {
            let mut pv = PVal::put(x.to_owned());
            hm.insert(pv.clone(),pv);
        }
        b.iter(||{
            for x in v.iter() {
                hm.get(&PVal::put(x.clone()));
            }
        });
    }
}

#[cfg(test)]
mod benches_unsf_str{
    use test::Bencher;
    use num::One;
    use num_bigint::{BigUint, RandBigInt};

    static SIZE: u64 = 10000;

    #[bench]
    fn a(b: &mut Bencher){
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..SIZE {
            v.push(rng.gen_biguint(64).to_string());
        }
        b.iter(||{
            for i in 0..(v.len()-1) {
                let x = &mut v[i];
                let x: *mut String = x;

                let y = &mut v[i+1];
                let y: *mut String = y;
                unsafe {
                    assert_ne!((*x),(*y));
                }
            }
        });
    }

    #[bench]
    fn bn(b: &mut Bencher){
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..SIZE {
            v.push(rng.gen_biguint(64).to_string());
        }
        b.iter(||{
            for i in 0..(v.len()-1) {
                let x =  &v[i];
                let y = &v[i+1];
                assert_ne!(x,y);
            }
        });
    }

    #[bench]
    fn c(b: &mut Bencher){
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..SIZE {
            v.push(rng.gen_biguint(64).to_string());
        }
        b.iter(||{
            for i in 0..(v.len()-1) {
                let x = v[i].clone();
                let y = v[i+1].clone();
                assert_ne!(x,y);
            }
        });
    }
    #[bench]
    fn d(b: &mut Bencher){
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..SIZE {
            v.push(rng.gen_biguint(64));
        }
        let one = BigUint::default();
        b.iter(||{
            for _ in 0..SIZE {
                for i in 0..(v.len()) {
                    let x = &mut v[i];
                    *x = &*x + &one;
                }
            }
        });
    }
    #[bench]
    fn e(b: &mut Bencher){
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..SIZE {
            v.push(rng.gen_biguint(64));
        }
        let one = BigUint::default();
        b.iter(||{
            for _ in 0..SIZE {
                for i in 0..(v.len()) {
                    let x = &mut v[i];
                    let x: *mut BigUint = x;
                    unsafe {
                        (*x) = &(*x) + &one;
                    }
                }
            }
        });
    }
}

fn tmp(foo: *mut BigUint){
   unsafe {
       (*foo) = &(*foo) + &BigUint::one();
   }
}

fn main() {
    let x = &mut "alo".to_string();
    let x: *mut String = x;

    let y = &mut "alo".to_string();
    let mut y: *mut String = y;
    println!("{:?}",y);
    unsafe {
        println!("{}",*y);
        assert_eq!((*x), (*y));
    }

    // let mut v = Vec::<KeyPair1>::new();
    // v[1] = KeyPair1::default();
    // println!("{:?}",v[1]);
    // let mut hm = HashMap4::new();
    // hm.insert(KeyPair1::new(BigUint::default(), BigUint::one()));
    // hm.insert(KeyPair1::new(BigUint::one(), BigUint::default()));
    // let x = hm.get(KeyPair1::default());
    // println!("{:?}",x);

    let mut v = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..1 {
        v.push(rng.gen_biguint(64));
    }
    println!("{}", v[0]);
    let x = &mut v[0];
    *x = &*x + &BigUint::one();
    println!("{}",x);
    println!("{}",v[0]);
    /*let mut v = Vec::new();
    for _ in 0..10 {
        let mut rng = rand::thread_rng();
        v.push(rng.gen_biguint(64));
    }
    let mut hm = HashMapx::new();
    for x in v.iter() {
        let mut pv = PVal::put(x.to_owned());
        hm.insert(pv.clone(), pv.clone());
    }

    println!("{:#?}", hm);

    let mut hm = HashMap::new();
    for x in v.iter() {
        let mut pv = PVal::put(x.to_owned());
        hm.insert(pv.clone(), pv);
    }
    println!("{:#?}", hm);*/
}