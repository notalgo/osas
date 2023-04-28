#![feature(test)]
#![feature(box_into_inner)]

mod hm;
mod pval;
mod pbst;
mod idk;
mod r#as;
mod hashes_test;
mod hm_try_4;
mod hm_try_5;

extern crate test;

use std::collections::HashMap;
use std::mem;
use mem::size_of;
use std::fs::File;
use std::io::Read;
use std::thread::spawn;
use num::{One, ToPrimitive, Zero};
use num_bigint::{BigUint, RandBigInt, ToBigUint};
use rand::thread_rng;
use osas::Generator;
use crate::hm::HashMapx;
use crate::hm_try_4::{HashMap4, KeyPair1};
use crate::hm_try_5::mhfx;
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
        /* let mut rng = rand::thread_rng();
         let mut vec;
         for _ in 0..10000 {

         }*/
        /*let mut x = AS::new();
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let k = rng.gen_biguint(64);
            let v = rng.gen_biguint(64);
            let k = PVal::put(k);
            let v = PVal::put(v);
            x.insert(KeyPair::new(k, v));
        }*/
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
                    let k = i.cone();
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

    static BITLEN: u64 = 1 << 14;

    #[bench]
    fn bn(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..SIZE {
            v.push(rng.gen_biguint(BITLEN));
        }
        let mut hm = HashMap4::new();
        b.iter(|| {
            for i in v.iter() {
                hm.insert(KeyPair1::new(i.clone(), Some(i.clone()), None, false));
                // let _ = hm.get(i);
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
                // hm.get(x);
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
mod numpt {
    use test::Bencher;
    use osas::Generator;

    static SIZE: u64 = 5;

    #[bench]
    fn mine(b: &mut Bencher) {
        let gn = Generator::init();
        b.iter(|| {
            for _ in 0..SIZE {
                let _ = gn.new_prime(512);
            }
        });
    }

    #[bench]
    fn default(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..SIZE {
                let _ = num_primes::Generator::new_prime(512);
            }
        });
    }
}

#[cfg(test)]
mod benches_unsf_str {
    use std::cmp::min;
    use std::collections::HashMap;
    use test::Bencher;
    use num::One;
    use num_bigint::{BigUint, RandBigInt};
    use num_traits::ToPrimitive;
    use rand::thread_rng;
    use crate::{any_as_u8_slice, genh, mhf};
    use crate::hashes_test::FNVa1;

    static SIZE: usize = 10000;


    /*    #[bench]
        fn a(b: &mut Bencher) {
            let mut v = Vec::new();
            let mut rng = rand::thread_rng();
            for _ in 0..SIZE {
                v.push(rng.gen_biguint(64).to_string());
            }
            for _ in 0..3 {
                b.iter(|| {
                    /*for _ in 0..SIZE {

                    }*/
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
        }
        #[bench]
        fn ab(b: &mut Bencher) {
            let mut v = Vec::new();
            let mut rng = rand::thread_rng();
            for _ in 0..SIZE {
                v.push(rng.gen_biguint(64).to_string());
            }
            for _ in 0..3 {
                b.iter(|| {
                    for i in 0..(v.len() - 1) {
                        let x = &v[i];
                        let x: *const String = x;

                        let y = &v[i + 1];
                        let y: *const String = y;
                        unsafe {
                            assert_ne!((*x), (*y));
                        }
                    }
                });
            }
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
        }*/
    /*
        #[bench]
        fn mhft(b: &mut Bencher) {
            let mut vec = Vec::<String>::new();
            let mut g = thread_rng();
            let mut flg = BigUint::default();
            for _ in 0..SIZE {
                let x = g.gen_biguint(1<<14);
                vec.push(x.to_string());
            }
            b.iter(|| {
                for i in vec.iter() {
                    let i = i.as_bytes();
                    genh(i);
                }
            });
        }*/


    #[test]
    fn mhftbtest() {
        let mut vec = Vec::<String>::new();
        let mut g = thread_rng();
        let mut flg = BigUint::default();
        for _ in 0..SIZE {
            let x = g.gen_biguint(1 << 14);
            vec.push(x.to_string());
        }
        let mut hm = HashMap::new();
        for i in vec.iter() {
            let ix = i.as_bytes();
            let k = genh(ix);
            if let Some(key) = hm.get(&k) {
                if *key != i {
                    println!("{}", key);
                }
            } else {
                hm.insert(k, i);
            }
        }
    }

    #[bench]
    fn mhftb1(b: &mut Bencher) {
        let mut vec = Vec::<String>::new();
        let mut g = thread_rng();
        let mut flg = BigUint::default();
        for _ in 0..SIZE {
            let x = g.gen_biguint(1 << 14);
            vec.push(x.to_string());
        }
        let mut hm = HashMap::new();
        b.iter(|| {
            for i in vec.iter() {
                unsafe {
                    hm.insert(genh(any_as_u8_slice(i)), i);
                }
            }
        });
    }

    #[bench]
    fn mhftb2(b: &mut Bencher) {
        let mut vec = Vec::<String>::new();
        let mut g = thread_rng();
        let mut flg = BigUint::default();
        for _ in 0..SIZE {
            let x = g.gen_biguint(1 << 14);
            vec.push(x.to_string());
        }
        let mut hm = HashMap::new();
        b.iter(|| {
            for i in vec.iter() {
                hm.insert(i, i);
            }
        });
    }

    #[bench]
    fn mhftb3(b: &mut Bencher) {
        let mut vec = Vec::<String>::new();
        let mut g = thread_rng();
        let mut flg = BigUint::default();
        for _ in 0..SIZE {
            let x = g.gen_biguint(1 << 14);
            vec.push(x.to_string());
        }
        let mut hm = HashMap::new();
        let mut fnv = FNVa1::init();
        b.iter(|| {
            for i in vec.iter() {
                unsafe {
                    hm.insert(fnv.hash(any_as_u8_slice(i)), i);
                }
            }
        });
    }

    #[bench]
    fn mhftb4(b: &mut Bencher) {
        let mut vec = Vec::<String>::new();
        let mut g = thread_rng();
        let mut flg = BigUint::default();
        for _ in 0..SIZE {
            let x = g.gen_biguint(1 << 14);
            vec.push(x.to_string());
        }
        let mut hm = HashMap::new();
        let mut fnv = FNVa1::init();
        b.iter(|| {
            for i in vec.iter() {
                unsafe {
                    hm.insert(genh(any_as_u8_slice(&fnv.hash(any_as_u8_slice(i)))), i);
                }
            }
        });
    }

    /*    #[bench]
        fn ls(b: &mut Bencher) {
            let mut v = Vec::new();
            let mut rng = rand::thread_rng();
            for _ in 0..SIZE {
                v.push(rng.gen_biguint(64).to_usize().unwrap());
            }
            b.iter(|| {
                for i in 0..v.len() {
                    let mut x = 0x1 << &(v[i]);
                    let _ = 0x1 & &(v[i]);
                    let _ = 0x1 | &(v[i]);
                    let _ = 0x1 ^ &(v[i]);
                }
            });
        }

        #[bench]
        fn lsu(b: &mut Bencher) {
            let mut v = Vec::new();
            let mut rng = rand::thread_rng();
            for _ in 0..SIZE {
                v.push(rng.gen_biguint(64).to_usize().unwrap());
            }
            b.iter(|| {
                for i in 0..v.len() {
                    let v: *const usize = &v[i];
                    unsafe {
                        let _ = 0x1 << &(*v);
                        let _ = 0x1 & &(*v);
                        let _ = 0x1 | &(*v);
                        let _ = 0x1 ^ &(*v);
                    }
                }
            });
        }

        #[bench]
        fn lsx(b: &mut Bencher) {
            let mut v = Vec::new();
            let mut rng = rand::thread_rng();
            for _ in 0..SIZE {
                v.push(rng.gen_biguint(64).to_usize().unwrap());
            }
            b.iter(|| {
                for i in 0..v.len() {
                    let _ = 1 << &(v[i]);
                    let _ = 1 & &(v[i]);
                    let _ = 1 | &(v[i]);
                    let _ = 1 ^ &(v[i]);
                }
            });
        }*/

    /*    #[bench]
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
        }*/
    /*
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
        fn h(b: &mut Bencher) {
            let mut v = Vec::new();
            let mut rng = rand::thread_rng();
            for _ in 0..SIZE {
                let x = rng.gen_biguint(64).to_string();
                v.push(x);
            }
            b.iter(|| {
                for i in v.iter() {
                    mhf(i.as_bytes());
                }
            });
        }*/
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

    for i in (0..len).step_by(2) {
        x ^= a[i] as usize;
        x %= 0xfffffff
        // x ^= a[i + 1] as usize;
    }
    /*for i in 0..(4 as usize) {
        if i < 2 {
            x = x ^ a[i] as usize;
        } else {
            x = x ^ a[len - i - 1] as usize;
        }
    }*/
    x
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    core::slice::from_raw_parts(
        (p as *const T) as *const u8,
        size_of::<T>(),
    )
}

fn gint(no: u32) -> u32 {
    (1 << (no & 31)) + (no >> 5)
}


fn bwadd(x: u32, y: u32) -> u32 {
    let mut x = x;
    let mut y = y;
    let mut c;
    while y != 0 {
        c = x & y;
        x ^= y;
        y = c << 1;
    }
    x
}

fn cls(n: u32, d: u32) -> u32 {
    (n << d) | (n >> (31 - d))
}

fn crs(n: u32, d: u32) -> u32 {
    return (n >> d) | (n << (32 - d));
}

fn genh(a: &[u8]) -> u32 {
    let mut flag = 0;
    // let mut flag1 = 0;
    // let mut flag2 = 0;
    let mut gintx;
    let mut ix;
    let len = a.len();
    for i in 0..len {
        /*if (i & 1) == 0 {
            // flag <<= 1;
            flag = crs(flag, 1);
        }*/ /*else {

        }*/
        ix = a[i] as u32;
        gintx = gint(ix);
        // flag1 |= gintx;
        flag = flag ^ gintx ^ ix;
        flag = cls(flag, gintx & 30);
        // let c = flag2 & gintx;
        // flag = flag ^ gintx;
        // flag2 = bwadd(flag2, gintx);
    }
    flag
    // bwadd(flag,flag2)
    // bwadd(flag, bwadd(flag1 << 3, flag1 << 1))
    // flag | (flag & !flag1)
    // bwadd(flag, bwadd(flag1 << 3, bwadd(flag1 << 1, flag2)))
    // flag + (flag1 << 3) + (flag1 << 1) + flag2
    // flag %32 + (flag1 << 3)%32 + (flag1 << 1) + flag2
}

/*fn genh(a: &[u8]) -> u32 {
    let mut flag = 0;
    // let mut flag1 = 0;
    // let mut flag2 = 0;
    let mut gintx;
    let mut ix;
    let len = a.len();
    for i in 0..len {
        /*if (i & 1) == 0 {
            // flag <<= 1;
            flag = crs(flag, 1);
        }*/ /*else {

        }*/
        ix = a[i] as u32;
        gintx = gint(ix);
        // flag1 |= gintx;
        flag = flag ^ gintx ^ ix;
        flag = cls(flag, gintx & 30);
        // let c = flag2 & gintx;
        // flag = flag ^ gintx;
        // flag2 = bwadd(flag2, gintx);
    }
    flag
    // bwadd(flag,flag2)
    // bwadd(flag, bwadd(flag1 << 3, flag1 << 1))
    // flag | (flag & !flag1)
    // bwadd(flag, bwadd(flag1 << 3, bwadd(flag1 << 1, flag2)))
    // flag + (flag1 << 3) + (flag1 << 1) + flag2
    // flag %32 + (flag1 << 3)%32 + (flag1 << 1) + flag2
}*/

/*fn genh(a: &[u8]) -> u32 {
    let mut flag = 0;
    let mut flag1 = 0;
    let mut flag2 = 0;
    for i in a {
        /*if (i & 1) == 0 {
            // flag <<= 1;
            flag = crs(flag, 1);
        }*/ /*else {

        }*/
        let gint = gint(*i as u32);
        flag ^= gint; //^ (*i as u32);
        flag = cls(flag, 1);
        flag1 |= gint;
        flag2 = bwadd(flag2, gint);
    }
    // flag | (flag & !flag1)
    bwadd(flag, bwadd(flag1 << 3, bwadd(flag1 << 1, flag2)))
    // flag + (flag1 << 3) + (flag1 << 1) + flag2
    // flag %32 + (flag1 << 3)%32 + (flag1 << 1) + flag2
}*/

fn main() {
    // println!("{}","000001001001000001100001000011");
    let x = b"000001001001000001100001000011";
    // let x: &mut [u8; 30] = &mut x;
    // x.reverse();
    let len = x.len();
    for i in (0..len).rev() {
        print!("{}", x[i] as char);
    }
    println!();
    // let mut foo = 19142723;
    let mut foo = 814090528;
    for i in 1..=20 {
        let bar = foo & 3;
        // println!("{}",bar);
        if bar == 0 {
            println!("{i}");
        } else {
            match bar {
                1 => {
                    println!("Fizz");
                }
                2 => {
                    println!("Buzz");
                }
                _ => {
                    println!("FizzBuzz");
                }
            }
        }
        foo = (foo >> 2) | (bar << 28);
    }
    /*let mut v = Vec::new();
    let mut hm = HashMap::new();
    for i in 0..256 {
        v.push(genh(&[i as u8]));
        if let Some(x) = hm.get(&v[i]) {
            hm.insert(v[i], x+1);
        }else {
            hm.insert(v[i], 1);
        }
    }
    for i in hm.iter() {
        let (key, val) = i;
        println!("{} {}",key, val);
        if *val > 1 {

        }
    }*/
    /*    let x = "njimp".to_string();
        let x = x.as_bytes();
        let y = "pbilk".to_string();
        let y = y.as_bytes();*/
    // println!("{} {}", genh(x), genh(y));
    let range = 1;
    let mut g = thread_rng();
    let mut f = File::open("assets/1.fa").unwrap();
    // let mut f = File::open("assets/foo.txt").unwrap();
    // let mut f = File::open("assets/keywords.txt").unwrap();
    // let mut f = File::open("assets/keywords1.txt").unwrap();
    // let mut f = File::open("assets/words.txt").unwrap();
    let mut string = String::new();
    f.read_to_string(&mut string).unwrap();
    let mut vec = Vec::new();
    // let string = "aloalo".to_string();
    let mut chars = string.as_bytes();
    let len = chars.len();
    let mut hm = HashMap::new();
    let mut hm1 = HashMap::new();
    for i in (0..len).step_by(5) {
        let stx = String::from_utf8((&[chars[i], chars[i + 1], chars[i + 2], chars[i + 3], chars[i + 4]]).to_vec()).unwrap();
        // println!("{}",stx);
        vec.push(stx);
    }
    let mut dup = 0 as usize;
    for _ in 0..range {
        for i in vec.iter() {
            let bytes = i.as_bytes();
            let x = genh(bytes);
            if let Some(c) = hm.get(&x) {
                if i != *c {
                    dup += 1;
                    println!("{} {}", i, c);
                }
            } else {
                hm.insert(x, i);
            }
            hm1.insert(i, i);
        }
    }
    println!("{} {} {}", dup, hm.len(), hm1.len());
    /*    let mut dup = 0 as usize;
        let mut hm = HashMap::new();
        let mut hmt = HashMap::new();
        for _ in 0..range {
            let mut flag = 0;
            let x = g.gen_biguint(1024);
            let bytes = x.to_bytes_le();
            let mut st = "".to_string();
            for i in bytes {
                st += &*(i as char).to_string();
                flag ^= gint(i as u32);
            }
            hmt.insert(x,1);
            hm.insert(flag, 1);
        }
        println!("{} {} {}", dup, hm.len(), hmt.len());*/
    /*    let range = 10000;
        let mut rng = rand::thread_rng();
        let mut hm = HashMap::new();
        let size = 1 << 8;
        for _ in 0..range {
            let x = rng.gen_biguint(size);
            unsafe {
                let hs = mhfx(any_as_u8_slice(&x));
                hm.insert(x,hs);
            }
        }
        let mut foo = Vec::new();
        let mut hmx = HashMap::new();
        for v in hm {
            let mut x = |xy:(BigUint,usize)| {
                if foo.contains(&xy.1) {
                    hmx.insert(xy.0,xy.1);
                }else {
                    foo.push(xy.1);
                }
            };
            x(v);
        }
        println!("{}", hmx.len());*/
    /*    unsafe {
            let x = BigUint::parse_bytes(b"93761645087621037457369093227850071270232282134456041814856327730631171877101546094279770239909775777768566697642985541247632119277495118648628480480101870942336175872074635048622858349088383446088924466868179837687231736958587257532266826121822184238335148295569188515971768853269002733227372786406935060616026917174556403218026729097256380661366660686019199228377344400824345564908096172409914464650549102914754728089441003498168492286590975206104846437745308694013554703184677215667715250003082550458104667522285898282560770988889387693795380024115749336335588714541605577271609087053007395381217349909084433073056937075935145622769428220713166659397420150695834343823941959558266702519171417993046692031120505157426545630838813537037031423204642027134042166054646470212900072786631150181939793763006377304334686317029163841895160103768585860206639602065112253849986885316248968647519966877874701730074000988912999052038643159506182821932155218397835627367661224596894708572832260772662095349861895748739711180276451038611519233409558300925651329195568258395317023780750845379832454512766840138663349753743657269425802293861308483732402735178988776361722044810753657499441546276813766332197477764213917310739809864601622056080533568756387919188676891395838946254883610258717132887688043518703886531338888868271264489491444711873851153496309420641818788654444830997000665469684933464915409511228547942116538715587959555356107360201731011112220448376894202036695417892112734709196725042424935174885204669676052372761994142114325631155063681811362579269926058932850158204391255885949170673168105367079820798048953800331791999231324521884939639258837857517364792290280259997304633707240522504232373247432819613124409409288723466652747564422076345690539313121277825326864169981740939896397686669804528126523561172065092287467511926397059931159571389919795567317568585754863044812126151882279043642825776539289096795854910889372675791252073929701357947298669145777304582488971118826838879455135600415714260980314515449746163874777337218450359736434758874075110472234693995373554353560530668070502459203267727890453323381975842685587353445460217435801035936171767737112849231571426254566123407258020454675634810998570506648969074407823956880841536695647200257477545937371667556678137007044128706195516907516756302981310543447476248102887937337572996201063938676348317392586733584350756842061993153934438645411262424406215091779829312603281774528769828418286594456859400226657470788871372264469551622272843244578634175308327761922485643593765774616333460164180037366846371451490333006166171565234052384136000313705052836227332065023153293517985262212364271100016049925481084180233949698068284446329084038986181971350573141602023469494324672423131250918982166420139679783678715317916120933709017823492972768537536771648004185107627993603934662075652679414015678704735558853279096627374223901412038311834385923409113335848703081288682086017953999288724187060340444255545931949555475783482793813259168136347011971082683693459529092711277919180980197911671641097772446609070698820567052408914855541031683164094308831979640727616169084182333376068118542728104538278067809299222961751800949324878703163367680182205830158842336786935700625180131465408758920560962982274587454271611662703472890317614521586602813269213282843261794433731211321466387932818193925637139311459653550658832275352044216143402942194520542187684875100561420978267092022019130915142625424119283081486673645746441022017358095062916512922165572029204942216082675921860728189510100101436965906479326136398430914019489094255974594839850867274633621356927691921448189901377624692910788882337099146346952199717705670883754380919974290618723288944914251679301114510173836294761010116691135737975592975923301812009945808460343318755550338556995703128772298668939674690226096034640434012753943751388463866388831086596527323673660711885989548406533011442799108445631580914192397896977334084568291791362452609064806143599547818052921031601722952495823423941881599692450105105246662923306828103033107201001118213885937284668494605759442009147784301813738849067218523565696513116015454627885872079626146480868702875666778811955138075444137251553901517182562086649400357744213816006030190166691943851428599378674323584748623065908516930847407233232427629825948002111085148478349334064991192239734546276159212650700708831631388928472271307164891212313204998127453935422768564243570759629976821098386889870485306000433616536879210186199421392390491432708714033358563159591796130682254327929436231471808803751554955612124346109666302214280785793108481240107644082252896822053666283404288218373161690253374227254187282349925459177006429322490206516093569130383199425090072708165684332604218217351539228427509762973009789146147193320369878519608382948551482476330324469926948874252854073305059502848177191450663481020711456214316095887542918399388962094598976770278384915587489149060526288", 10).unwrap();
            let x = any_as_u8_slice(&x);
            let y = BigUint::parse_bytes(b"978214294526633021980207400810719256322648091909487925930696615722004340224093661510221783297689796737568695723194013176639441627009597639340633572841836224191460376522825165323077928196476323844872392876435750595065564517356618600570918294632997354945964277798782152307989031606115450445946612130820943914689178729563339653835110472100796329445614101643920616597311383792578680918778676460329210187298570056720650652336055897276781540057609328525951393038393092402938194792717950291909472288308767630179904360776546689370444872979848811883186911785797673171129003160295147578866580654766754294182422979156987397195904322973720727730819661336757862468600616253292123834739609881222473878990792120044864429004841570383163035932788675114816712341306534206283430607597099525883884007552599742859802351110374998290864984777353717585871678581384158124880345441142630536714462174798129367172673266037472550546891399444356229878585193443270170555637903894942795977173266980185561446516331626578261954581100664837045640793490821067488145482639318792440944894212997978182766387620081859388037932111774442779371314905566089350749144473221036464141450042867440063572942328873524661841979909854865523987086280757529185095904264929178479910690823506129766845142696800946099968840788891469253887882134805490560018420807740548011070051438209597135613690288752029976450820256876680534014914671537036050226553505843424386068288540792958355912853866366577631609876547638849117859601319719281501861985491071795620688775883134451564978953713027438952991607131456337940371303215661124769531296825439697566421549033393238046965479967475022606849213518206795955037176393282523896875471145546496825945421087548723710003944379150797817838404504166247260480724175861975064977516248976266494268534145067783262499127206028798895430095435967622041481549686248886710022245713579947618802150138792714499832102378495647999242194607714902377848254676596347302499456561231291154527790339396325427184119827562892472023593991645758435675719768249614382378981621629384093705259263651258301936352129602528763974707267710461862529426088135021724341427173879434679408617474572098551199287819808151037594307211259212763578932321640238010106147228574788895001118070580794765891936873252413549273508625084973020670458846231612494205476221870622023401353228385434854360193867658954623562832700347519333283531895221308808244955159131666637988345349827755550395880344399171962822820836195621393968271643298415450890879646059797336135205162828351184945344082789086951896162319433640244287981433042245678214283513914468508747978175317378694066039300863387926137745104284539310871356094361372597605322799870966122613267635448492004236473745373658534251001154976776406355591510156729307849334190343993276892334346662682932487936680517700472971618232491697585578635610926639229570207683039021401106105380512978691556615431130322998691914600121273264153709981976914370033340676230824905074543099778419544889606210969857336713217289000645958103385535064492101180032208150768685204362796833073840685963212889547212188448599953583367496416074780990068423536577000459043924318395771182070631073485164456006636243264675710311342900720128097883265415555240767342378166690691698020751631837409336223998293811205413553447779166299431417420172231945503463197482699268829064011485851410641635388463365759343158365073952099399250249489090598602017894525282789396853749144061079253030984663632242870509761772313321977187199443926898654734707053429092587463250917354118162694746862325571120651007104941700877731953822077574073395403811331280157259799604570646125783974807059042930028715678446201365188987479085523717521389362036977570575145442234899036882506896480681361432743848501284519164939296620206118721749279904304450806276549425641353208236042018628265227645547208795819229814552364674037153480719907956710145789203324180289833565208341949339540034816455032415871028093212041445305080168671509439016724846323718192152986819986480135592962908093874684164468390484939113748394389999555248511858056483995851304996092014938541234389023368658444713341521884016821512323759428616032233075086200568952677337493832354535017949232851195181017612646730700711926691235831932160766914288224045085043868241001611853435049405250606360313894375858639500155550723866780476913285557803780580945074032682505126021055261813805685633525687251519823033858708540219829593466440979363675859821816903103136007898241687396810045484973878329549143697553897404381641843902262405622343912083032967546046124431125767344846599619072779832081276400748604006610492608784511435812250824127924930393902304433294806619559531217892345827769162916277567691107447082884340640224933171667936985709389483580222697607835040975108688486971674930953425419953337509232970946541178954923002542438594865374699966147575123225480985627068268200835966346697951974734487554482565719281327915669156969571585352955223023756014590313747272438", 10).unwrap();
            let y = any_as_u8_slice(&y);
            println!("{}", y == x);
            println!("{}", mhf(y) == mhf(x));
            println!("{}", mhf(y));
            println!("{}", mhf(x));
        }*/

    /* let mut v = Vec::new();
     let mut rng = rand::thread_rng();
     let range = 10;
     let first = rng.gen_biguint(1 << 8);
     for i in 0..range {
         if i == 0 {
             v.push(first.clone());
             v.push(first.clone());
         }
         v.push(rng.gen_biguint(1 << 8));
     }
     let mut hm = HashMap4::new();
     let mut xx:usize = 0;
     for i in v.iter() {
         hm.insert(KeyPair1::new(i.clone(), Some(i.clone()), None, false));
         let x = hm.get(i);
         // println!("{}",x);
         assert_eq!(x,Some(i));
         xx += 1;
     }
     println!("{}",hm);
     println!("{}",xx);

     unsafe {
         println!("{}",DC);
     }
 */
    /* println!("{}", (1<<14)/(1<<3));
     //651659899
     println!("{:x}", mhf("ssdd".to_string().as_bytes()) + 10);
     println!("{}", mhf("ssddd".to_string().as_bytes()));
     println!("{}", mhf("ssdddssddd".to_string().as_bytes()));*/

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