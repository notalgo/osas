use core::mem::size_of;
use std::ops::{BitOr, Shl};
use num_bigint::BigUint;

struct HashMap5 {
    l: Vec<KeyPair1>,
}

pub struct KeyPair1 {
    key: BigUint,
    hash: usize,
    val: Option<BigUint>,
    is_node: bool,
}

unsafe fn rls(x: u32, shift: u32) -> u32 {
    let shift = shift % 32;
    (x << shift) | (x >> (32 - shift)%32)
}


pub fn mhfx(a: &[u8]) -> usize {
    let len: *mut usize = &mut a.len();
    unsafe {
        if &(*len) < &4 {
            let mut x = u32::from_le_bytes(a.try_into().unwrap());
            for i in 0..(*len) {
                x ^= a[i] as u32;
            }
            return x as usize;
        }
        let lbt: *mut usize = &mut ((*len) >> 1);
        let m = u32::from_le_bytes([a[((*lbt) - 1)], a[(*lbt)], a[(*lbt) + 1], a[(*lbt) + 2]]);
        let l = u32::from_le_bytes([a[((*len) - 4)], a[(*len) - 3], a[(*len) - 2], a[(*len) - 1]]);
        let s = u32::from_le_bytes([a[(0)], a[1], a[2], a[3]]);
        let x: *mut u32 = &mut (rls(s, rls(m, rls(l, (*len) as u32))));
        return *x as usize;
    }
}

