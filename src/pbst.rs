use std::cmp::Ordering::*;
use num::{One};
use num_bigint::BigUint;

#[derive(Debug,Default,Clone)]
struct Data{
    key: BigUint,
    is_node: bool,
    val: Option<BigUint>,
    valn: Option<Box<BST>>,
}

#[derive(Debug,Default,Clone)]
pub struct Node {
    hash: BigUint,
    left: BST,
    right: BST,
    data: Box<Data>
}

#[derive(Debug,Default,Clone)]
pub struct BST(Option<Box<Node>>);



impl BST {
    pub fn new() -> BST {
        BST(None)
    }
    fn pt(&self, tree: *mut BST) {
        unsafe {
            let node = match (*tree).0.clone() {
                Some(x) => {
                    x
                }
                None => {
                    return;
                }
            };
            self.pt(&mut node.left.clone());
            let key = node.data.key.clone();
            let val = match node.data.val.clone() {
                Some(x) => {
                    x
                }
                None => {
                    self.pt(&mut Box::into_inner(node.data.valn.clone().unwrap()));
                    return;
                }
            };
            println!("{}: {},",key, val);
            self.pt(&mut node.right.clone());
        }
    }
    pub fn insert(&mut self, key: BigUint, val: &mut BigUint) -> bool {
        self.insert_hash(key.clone(), key, Some(val), None, false)
    }

    pub fn insert_node(&mut self, key: BigUint, val: &mut BST) -> bool{
        self.insert_hash(key.clone(), key, None, Some(val),true)
    }

    fn insert_hash(&mut self, hash: BigUint, key: BigUint, val: Option<&mut BigUint>, valn: Option<&mut BST>, is_node: bool) -> bool{
        let mut hash = hash;
        let mut tree: *mut BST = self;
        unsafe {
            while let Some(ref mut node) = (*tree).0 {
                match hash.to_owned().cmp(&node.hash) {
                    Less => tree = &mut node.left,
                    Greater => tree = &mut node.right,
                    Equal => {
                        return if is_node {
                            let x = (*tree).0.as_mut().unwrap().data.as_mut();
                            x.valn.insert(Box::new(valn.unwrap().clone()));
                            x.is_node = true;
                            x.val = None;
                            return false;
                            if key.clone() == (*tree).0.clone().unwrap().data.key.clone() {
                                (*tree).0.clone().unwrap().data.valn = Some(Box::new(valn.unwrap().to_owned()));
                                false
                            } else {
                                while self.contains(hash.clone()) {
                                    hash += BigUint::one();
                                }
                                self.insert_hash(hash.to_owned(), key.clone(), val, valn, true);
                                false
                            }
                        } else {
                            let x = (*tree).0.as_mut().unwrap().data.as_mut();
                            x.val.insert(val.unwrap().clone());
                            x.is_node = false;
                            x.valn = None;
                            return false;
                            if key.clone() == (*tree).0.clone().unwrap().data.key.clone() {
                                println!("here");

                                /*let mut v:*mut Node = (*tree).0.as_mut().unwrap().as_mut();
                                let mut data:*mut Data = (*v).data.as_mut();
                                (*data).val = Some(val.unwrap().clone());*/
                                false
                            } else {
                                // todo!("increase hash value till it's unique")
                                while self.contains(hash.clone()) {
                                    hash += BigUint::one();
                                }
                                self.insert_hash(hash.to_owned(), key.clone(), val, valn, false);
                                false
                            }
                        }
                    }
                }
            }
            if is_node {
                (*tree).0 = Some(Box::new(Node {
                    hash,
                    left: BST(None),
                    right: BST(None),
                    data: Box::new(Data {
                        key,
                        is_node,
                        val: None,
                        valn: Some(Box::new(valn.unwrap().to_owned())),
                    }),
                }));
            }else {
                (*tree).0 = Some(Box::new(Node {
                    hash,
                    left: BST(None),
                    right: BST(None),
                    data: Box::new(Data {
                        key,
                        is_node,
                        val: Some(val.unwrap().to_owned()),
                        valn: None,
                    }),
                }));
            }
            true
        }
    }

    pub fn contains(&self, key: BigUint) -> bool {
        let mut tree = self;
        while let Some(ref node) = tree.0 {
            match key.cmp(&node.hash) {
                Less => tree = &node.left,
                Greater => tree = &node.right,
                Equal => return true,
            }
        }
        false
    }
    pub fn remove(&mut self, key: BigUint) -> bool {
        let mut tree: *mut BST = self;
        unsafe {
            while let Some(ref mut node) = (*tree).0 {
                match key.cmp(&node.hash) {
                    Less => tree = &mut node.left,
                    Greater => tree = &mut node.right,
                    Equal => {
                        (*tree).0 = None;
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod pbst_tests{
    use num_bigint::BigUint;
    use crate::pbst::BST;

    #[test]
    fn pbst_bst_test(){
        let mut tree = BST::new();
        let mut x = BigUint::from(69 as u64);
        tree.insert(x.clone(), &mut x);
        let mut x = BigUint::from(169 as u64);
        tree.insert(x.clone(), &mut x);
        let mut y = BST::new();
        y.insert(x.clone(), &mut BigUint::default());
        tree.insert_node(x.clone(), &mut y);
        tree.pt(&mut tree.clone());
    }
}