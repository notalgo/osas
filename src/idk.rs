use std::cmp::Ordering;
use num_bigint::BigUint;
use crate::pbst::BST;
use crate::pval::PVal;

#[derive(Clone,Default,Debug,Eq,PartialEq)]
struct FooData{
    key: PVal,
    value: PVal,
    is_node: bool
}

#[derive(Clone,Default,Debug,Eq)]
struct FooNode{
    left: FooBST,
    right: FooBST,
    data: Box<FooData>
}

impl PartialEq for FooNode{
    fn eq(&self, other: &Self) -> bool {
        self.data.key == other.data.key
    }
}

#[derive(Clone,Default,Debug,Eq,PartialEq)]
pub struct FooBST(Option<Box<FooNode>>);

impl FooBST {
    pub fn new() -> FooBST {
        FooBST(None)
    }
    pub fn insert(&mut self, key: PVal, val: PVal) -> bool {
        let mut tree:*mut FooBST = self;
        unsafe {
            while let Some(ref mut node) = (*tree).0{
                match key.cmp(&node.data.key) {
                    Ordering::Less => tree = &mut node.left,
                    Ordering::Greater => tree = &mut node.right,
                    Ordering::Equal => {
                        (*tree).0.as_mut().unwrap().data.as_mut().value = val.clone();
                        return false;
                    }
                }
            }
            (*tree).0 = Some(Box::new(FooNode{
                left: FooBST(None),
                right: FooBST(None),
                data: Box::new(
                    FooData{
                        key,
                        value: val,
                        is_node: false,
                    }
                ),
            }));
        }
        return true;
    }
    pub fn get(&mut self, key: PVal) -> Option<PVal>{
        let mut tree:*mut FooBST = self;
        unsafe {
            while let Some(ref mut node) = (*tree).0{
                match key.cmp(&node.data.key) {
                    Ordering::Less => tree = &mut node.left,
                    Ordering::Greater => tree = &mut node.right,
                    Ordering::Equal => {
                        return Some(node.data.value.clone());
                    }
                }
            }
        }
        return None;
    }
    pub fn remove(&mut self, key: PVal) -> bool {
        let mut tree:*mut FooBST = self;
        unsafe {
            while let Some(ref mut node) = (*tree).0{
                match key.cmp(&node.data.key) {
                    Ordering::Less => tree = &mut node.left,
                    Ordering::Greater => tree = &mut node.right,
                    Ordering::Equal => {
                        (*tree).0 = None;
                        return true;
                    }
                }
            }
        }
        return false;
    }
    pub fn contains(&mut self, key: PVal) -> bool{
        let x = self.get(key);
        match x {
            None => false,
            Some(_) => true
        }
    }
}

#[cfg(test)]
mod idk_tests{
    use num_bigint::BigUint;
    use crate::idk::FooBST;
    use crate::pval::PVal;

    #[test]
    fn test_all() {
        let mut x = FooBST::new();
        let hun = PVal::put(BigUint::from(100 as u64));
        let ten = PVal::put(BigUint::from(10 as u64));
        println!("{}",x.insert(hun.clone(), ten.clone()));;
        // x.insert(hun.clone(), hun.clone());
        x.insert(PVal::put(BigUint::default()), hun.clone());
        // println!("{:#?}", x);
        println!("{}", x.get(hun).unwrap().get());
        println!("{}", x.get(PVal::put(BigUint::default())).unwrap().get());
    }
}