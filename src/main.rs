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
use std::mem;
use mem::size_of;
use num::{One, ToPrimitive, Zero};
use num_bigint::{BigUint, RandBigInt, ToBigUint};
use crate::hm::HashMapx;
use crate::hm_try_4::{HashMap4, KeyPair1};
use crate::pval::PVal;
use crate::r#as::{AS, KeyPair};

fn btoi(bytes: Vec<u8>) -> BigUint {
    let mut r = BigUint::zero();
    for byte in bytes {
        r = (r << 8) | BigUint::from(byte);
    }
    r
}

fn itob(int: &mut BigUint) -> Vec<u8> {
    let mut int = int.clone();
    let len = ((int.to_bytes_be().len() << 3) + 7) >> 3;
    let b256 = BigUint::from(256 as u16);
    let mut r = vec![0; len].into_boxed_slice();
    for i in 0..len {
        let idk = (int.clone() % b256.clone()).to_bytes_be();
        r[len - i - 1] = idk[0];
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
mod tests {
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
    fn lin() {
        let mut x = AS::new();
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
    fn foo() {
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
    fn hmt() {
        // BigUint::from();
        let mut x = HashMapx::new();
        let ten = PVal::put(BigUint::from(10 as u64));
        x.insert(ten.clone(), PVal::put(BigUint::one()));
        println!("{}", x.get(ten).unwrap().get());
    }

    #[bench]
    fn foox(b: &mut Bencher) {
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
mod fb {
    use test::Bencher;

    #[bench]
    fn fb_4(b: &mut Bencher) {
        let r = 1000;
        b.iter(|| {
            for i in 1..=r {}
        });
    }

    #[bench]
    fn fb_3(b: &mut Bencher) {
        let r = 1000;
        b.iter(|| {
            let mut x = 1;
            let mut y = 1;
            let mut o;
            for i in 1..=r {
                o = true;
                if x == 3 {
                    // assert!(x%15!=0);
                    assert!(i % 3 == 0);
                    // assert!(x%5!=0);
                    x = 0;
                    o = false;
                }
                if y == 5 {
                    // assert!(x%15!=0);
                    // assert!(x%3!=0);
                    assert!(i % 5 == 0);
                    // print!("Buzz");
                    y = 0;
                    o = false;
                }
                if o {
                    assert!(i % 15 != 0);
                    assert!(i % 3 != 0);
                    assert!(i % 5 != 0);
                }
                x += 1;
                y += 1;
                // println!();
            }
        });
    }

    #[bench]
    fn fb_2(b: &mut Bencher) {
        let r = 1000;
        b.iter(|| {
            let mut o;
            for x in 1..=r {
                o = true;
                if x % 3 == 0 {
                    assert!(x % 3 == 0);
                    o = false;
                }
                if x % 5 == 0 {
                    // print!("Buzz");
                    assert!(x % 5 == 0);
                    o = false;
                }
                if o {
                    assert!(x % 15 != 0);
                    assert!(x % 3 != 0);
                    assert!(x % 5 != 0);
                }
                // println!();
            }
        });
    }

    #[bench]
    fn fb_1(b: &mut Bencher) {
        let r = 1000;
        b.iter(|| {
            for x in 1..=r {
                if x % 15 == 0 {
                    assert!(x % 15 == 0);
                } else if x % 3 == 0 {
                    assert!(x % 3 == 0);
                } else if x % 5 == 0 {
                    assert!(x % 5 == 0);
                } else {
                    assert!(x % 15 != 0);
                    assert!(x % 3 != 0);
                    assert!(x % 5 != 0);
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
    use crate::hm_try_4::{HashMap4, KeyPair1};
    use crate::idk::FooBST;
    use crate::pval::PVal;
    use crate::r#as::{AS, KeyPair};

    static SIZE: u64 = 10000;

/*    #[bench]
    fn a(b: &mut Bencher) {
        let mut x = AS::new();
        let mut rng = rand::thread_rng();
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..SIZE {
            v.push(rng.gen_biguint(64));
        }
        b.iter(|| {
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
    fn x(b: &mut Bencher) {
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
        b.iter(|| {
            for x in v.iter() {
                tree.get(PVal::put(x.clone()));
            }
        });
    }*/

    static BITLEN: u64 = 1<<14;

    #[bench]
    fn bn(b: &mut Bencher){
        let mut rng = rand::thread_rng();
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..SIZE {
            v.push(rng.gen_biguint(BITLEN));
        }
        let mut hm = HashMap4::new();
        for i in v.iter() {
        }
        b.iter(|| {
            for i in v.iter() {
                hm.insert(KeyPair1::new(i.clone(), Some(i.clone()), None, false));
                let _ = hm.get(i);
            }
        });
    }

    #[bench]
    fn y(b: &mut Bencher) {
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..SIZE {
            v.push(rng.gen_biguint(BITLEN));
        }
        let mut hm = HashMap::new();
        for x in v.iter() {
            // let mut pv = PVal::put(x.to_owned());

        }
        b.iter(|| {
            for x in v.iter() {
                hm.insert(x.clone(), x.clone());
                hm.get(x);
            }
        });
    }

/*    #[bench]
    fn z(b: &mut Bencher) {
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..SIZE {
            v.push(rng.gen_biguint(64));
        }
        let mut hm = AvlTreeMap::new();
        for x in v.iter() {
            let mut pv = PVal::put(x.to_owned());
            hm.insert(pv.clone(), pv);
        }
        b.iter(|| {
            for x in v.iter() {
                hm.get(&PVal::put(x.clone()));
            }
        });
    }*/
}

#[cfg(test)]
mod benches_unsf_str {
    use std::cmp::min;
    use test::Bencher;
    use num::One;
    use num_bigint::{BigUint, RandBigInt};
    use crate::mhf;

    static SIZE: u64 = 10000;

    #[bench]
    fn a(b: &mut Bencher) {
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..SIZE {
            v.push(rng.gen_biguint(64).to_string());
        }
        b.iter(|| {
            for i in 0..(v.len() - 1) {
                let x = &mut v[i];
                let x: *mut String = x;

                let y = &mut v[i + 1];
                let y: *mut String = y;
                unsafe {
                    assert_ne!((*x), (*y));
                }
            }
        });
    }

    #[bench]
    fn bn(b: &mut Bencher) {
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..SIZE {
            v.push(rng.gen_biguint(64).to_string());
        }
        b.iter(|| {
            for i in 0..(v.len() - 1) {
                let x = &v[i];
                let y = &v[i + 1];
                assert_ne!(x, y);
            }
        });
    }

    #[bench]
    fn c(b: &mut Bencher) {
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..SIZE {
            v.push(rng.gen_biguint(64).to_string());
        }
        b.iter(|| {
            for i in 0..(v.len() - 1) {
                let x = v[i].clone();
                let y = v[i + 1].clone();
                assert_ne!(x, y);
            }
        });
    }

    #[bench]
    fn f(b: &mut Bencher) {
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..SIZE {
            v.push(rng.gen_biguint(64));
        }
        b.iter(|| {
            for i in 0..(v.len() - 1) {
                assert_ne!(&v[i], &v[i + 1])
            }
        });
    }

    #[bench]
    fn g(b: &mut Bencher) {
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..SIZE {
            v.push(rng.gen_biguint(64));
        }
        b.iter(|| {
            for i in 0..(v.len() - 1) {
                let x: *mut BigUint = &mut v[i];
                let y: *mut BigUint = &mut v[i + 1];
                unsafe {
                    assert_ne!((*x), (*y));
                }
            }
        });
    }

    #[bench]
    fn d(b: &mut Bencher) {
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..SIZE {
            v.push(rng.gen_biguint(64));
        }
        let one = BigUint::default();
        b.iter(|| {
            for i in 0..(v.len()) {
                let x = &mut v[i];
                *x = &*x + &one;
            }
            /* for _ in 0..SIZE {

             }*/
        });
    }

    #[bench]
    fn e(b: &mut Bencher) {
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..SIZE {
            v.push(rng.gen_biguint(64));
        }
        let one = BigUint::default();
        b.iter(|| {
            for i in 0..(v.len()) {
                let x = &mut v[i];
                let x: *mut BigUint = x;
                unsafe {
                    (*x) = &(*x) + &one;
                }
            }
            /*for _ in 0..SIZE {

            }*/
        });
    }
    #[bench]
    fn h(b: &mut Bencher){
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..SIZE {
            let x = rng.gen_biguint(64).to_string();
            v.push(x);
        }
        b.iter(||{
            for i in v.iter() {
                mhf(i.as_bytes());
            }
        });
    }
}

fn tmp(foo: *mut BigUint) {
    unsafe {
        (*foo) = &(*foo) + &BigUint::one();
    }
}

pub fn mhf(a: &[u8]) -> usize {
    let len = a.len();
    let mut x = (0 as usize);
    if len < 4 {
        for i in 0..len {
            x = (x << 8) | a[i] as usize;
        }
        if x == 3 {
            x = x ^ a[0] as usize;
            x = x ^ a[1] as usize;
            x = x ^ a[2] as usize;
        }
        return x % 0xfffffff;
    }
    // x = x % 0xfffffff;
    for i in 0..(4 as usize) {
        if i < 2 {
            x = (x << 8) | a[i] as usize;
        } else {
            x = (x << 8) | a[len - i - 1] as usize;
        }
    }

    for i in (0..(len-1)).step_by(2) {
        x ^= a[i] as usize;
    }
    /*for i in 0..(4 as usize) {
        if i < 2 {
            x = x ^ a[i] as usize;
        } else {
            x = x ^ a[len - i - 1] as usize;
        }
    }*/
    x % 0xfffffff
}

fn main() {
    println!("{}", (1<<14)/(1<<3));
    //651659899
    println!("{}", mhf("ssd".to_string().as_bytes()));
    println!("{}", mhf("ssddd".to_string().as_bytes()));
    println!("{}", mhf("ssdddssddd".to_string().as_bytes()));

/*    let x = &mut BigUint::parse_bytes(b"7463749812302340912745859", 10).unwrap();
    let x: *mut BigUint = x;

    unsafe {
        (*x) = &(*x) % 20 as u64;
    }

    println!("{:?}", size_of::<HashMap4>());
    println!("{:?}", size_of::<KeyPair1>());
*/
    // let y = BigUint::from(20 as u64);
    // println!("{}", x%y);
/*    let x = &mut "alo".to_string();
    let x: *mut String = x;

    let y = &mut "alo".to_string();
    let mut y: *mut String = y;
    // println!("{:?}",y);
    unsafe {
        // println!("{}",*y);
        assert_eq!((*x), (*y));
    }*/

    // let mut v = Vec::<KeyPair1>::new();
    // v[1] = KeyPair1::default();
    // println!("{:?}",v[1]);
/*    let mut hm = HashMap4::new();
    hm.insert(KeyPair1::new(BigUint::default(), Some(BigUint::one()), None,false));
    hm.insert(KeyPair1::new(BigUint::one(), Some(BigUint::default()), None, false));
    let x = hm.get(&BigUint::default());
    let x = hm.get(&BigUint::one());
    println!("{:?}", x);

    let mut v = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..1 {
        v.push(rng.gen_biguint(64));
    }
    println!("{}", v[0]);
    let x = &mut v[0];
    *x = &*x + &BigUint::one();
    println!("{}", x);
    println!("{}", v[0]);*/
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