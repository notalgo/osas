use num::ToPrimitive;
use num_bigint::BigUint;
use crate::mhf;

#[derive(Debug,Default,Clone)]
pub struct HashMap4 {
    l: Vec<KeyPair1>,
}

#[derive(Default, Debug,Clone)]
pub struct KeyPair1 {
    key: BigUint,
    val: Option<BigUint>,
    is_node: bool,
    node: Option<Box<HashMap4>>
}

impl KeyPair1 {
    pub fn new(key: BigUint, val: Option<BigUint>, node: Option<Box<HashMap4>>, is_node: bool) -> Self {
        Self {
            key,
            val,
            is_node,
            node,
        }
    }
}

impl HashMap4 {
    pub fn new() -> Self {
        let mut v = vec![KeyPair1::default(); 0xfffffff];
        Self { l: v }
    }
    pub fn insert(&mut self, pair: KeyPair1) {
        let v = &mut self.l;
        unsafe {
            let x = mhf(any_as_u8_slice(&pair.key));
            v[x] = pair;
        }
    }
    pub fn get(&mut self, key: &BigUint) -> &KeyPair1 {
        let v = &self.l;
        unsafe {
            &v[mhf(any_as_u8_slice(key))]
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