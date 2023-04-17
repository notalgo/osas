use std::collections::LinkedList;
use std::fmt::{Display, Formatter};
use num::ToPrimitive;
use num_bigint::BigUint;

#[derive(Default, Clone)]
pub struct HashMap4 {
    l: Vec<KeyPair1>,
}

impl Display for HashMap4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in &self.l {
            if &i.hash != &0 {
                write!(f,"{}", i).unwrap();
            }
        }
        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct KeyPair1 {
    key: BigUint,
    hash: usize,
    val: Option<BigUint>,
    is_node: bool,
    node: Option<Box<HashMap4>>,
    next_node: LinkedList<Box<KeyPair1>>
}

impl Display for KeyPair1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}\n", self.hash, self.key)
    }
}

impl KeyPair1 {
    pub fn get_key(&self) -> &BigUint{
        &self.key
    }
    pub fn new(key: BigUint, val: Option<BigUint>, node: Option<Box<HashMap4>>, is_node: bool) -> Self {
        unsafe {
            let hash= mhf(any_as_u8_slice(&key));
            // println!("{}: {}",hash, key);
            Self {
                key,
                hash,
                val,
                is_node,
                node,
                next_node: LinkedList::default(),
            }
        }
    }
}

impl HashMap4 {
    pub fn new() -> Self {
        let mut v = vec![KeyPair1::default(); 0xfffffff];
        Self { l: v }
    }
    pub fn insert(&mut self, pair: KeyPair1) {
        let v:*mut Vec<KeyPair1> = &mut self.l;
        unsafe {
            let hash= pair.hash;
            if &(*v)[hash].hash != &0 {
                let _ = &(*v)[hash].next_node.push_back(Box::new(pair));
                return;
            }
            (*v)[hash] = pair;
        }
    }

    pub fn get(&mut self, key: &BigUint) -> Option<&KeyPair1> {
        let v: *mut Vec<KeyPair1> = &mut self.l;
        unsafe {
            let mut hash = mhf(any_as_u8_slice(key));
            if &(*v)[hash].key == key {
                return Some(&(*v)[hash]);
            }else {
                hash += 1;
                while &(*v)[hash].key != key {
                    hash += 1;
                    if &hash >= &0xfffffff {
                        return None;
                    }
                }
                return Some(&(*v)[hash]);
            }
            /*let mut hash = mhf(any_as_u8_slice(&key));
            let mut key = key;
            let key: *mut BigUint = &mut key;
            let x = &(*v)[hash];
            let end:*mut usize = &mut 0xfffffff;

            if x != &(*key) {
                while &hash <= &(*end) {
                    let mut vx:*mut KeyPair1 = &mut (*v)[hash];
                    let mut keyx = (*vx).key;
                    let keyx:*mut BigUint = &mut keyx;
                    if &(*keyx) == &(*key){

                    }
                }
                return x;
            }else {
                return x;
            }*/
        }
    }
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    core::slice::from_raw_parts(
        (p as *const T) as *const u8,
        core::mem::size_of::<T>(),
    )
}

fn hash(key: &BigUint) -> usize {
    let x = BigUint::from(100000 as u64);
    (*&key % &x).to_usize().unwrap()
}

fn mhf(a: &[u8]) -> usize {
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
        return x / 0xfffffff;
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
        x ^= (a[i] ^ a[i+1]) as usize;
        // x /= 0xfffffff;
        // x ^= a[i + 1] as usize;
    }
    let len:u8 = 12;
    ((x >> len) | (x << len)) % 0xfffffff
}