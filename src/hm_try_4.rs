use num::ToPrimitive;
use num_bigint::BigUint;

pub struct HashMap4 {
    l: Vec<KeyPair1>,
}

#[derive(Default, Debug)]
pub struct KeyPair1 {
    key: BigUint,
    val: BigUint,
}

impl KeyPair1 {
    pub fn new(key: BigUint, val: BigUint) -> Self {
        Self {
            key,
            val,
        }
    }
}

impl HashMap4 {
    pub fn new() -> Self {
        let mut v = Vec::new();
        for _ in 0..100000 {
            v.push(KeyPair1::default())
        }
        Self { l: v }
    }
    pub fn insert(&mut self, pair: KeyPair1) {
        // let v = &mut self.l;
        // v[hash(&pair.key)] = pair;
    }
    pub fn get(&mut self, key: KeyPair1) -> &KeyPair1 {
        let v = &self.l;
        &v[hash(&key.key)]
    }
}

fn hash(key: &BigUint) -> usize {
    let x = BigUint::from(100000 as u64);
    (*&key % &x).to_usize().unwrap()
}