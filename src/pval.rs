use num::ToPrimitive;
use num_bigint::BigUint;

#[derive(Default, Debug, Clone)]
pub struct PVal{
    val: BigUint,
    lt: usize
}

impl PVal{
    fn put(val: BigUint) -> Self{
        PVal{ val: val.clone(), lt: (val%BigUint::from(100 as u64)).to_usize().unwrap() }
    }
    pub fn get(&self) -> BigUint{
        self.val.clone()
    }

    pub fn lt(&self) -> usize {
        self.lt.clone()
    }

}