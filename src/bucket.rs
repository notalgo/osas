use std::cmp::Ordering::*;

#[derive(Debug)]
pub struct Node<K: Ord> {
    key: K,
    left: BST<K>,
    right: BST<K>,
}

#[derive(Debug)]
pub struct BST<K: Ord>(Option<Box<Node<K>>>);

impl<K: Ord> BST<K> {
    pub fn new() -> BST<K> {
        BST(None)
    }
    pub fn insert(&mut self, key: K) -> bool {
        let mut tree: *mut BST<K> = self;
        unsafe {
            while let Some(ref mut node) = (*tree).0 {
                match key.cmp(&node.key) {
                    Less => tree = &mut node.left,
                    Greater => tree = &mut node.right,
                    Equal => return false
                }
            }
            (*tree).0 = Some(Box::new(Node {
                key,
                left: BST(None),
                right: BST(None),
            }));
            true
        }
    }

    pub fn remove(&mut self, key: K) -> bool {
        let mut tree: *mut BST<K> = self;

        unsafe {
            while let Some(ref mut node) = (*tree).0 {
                match key.cmp(&node.key) {
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

    pub fn contains(&self, key: K) -> bool {
        let mut tree = self;
        while let Some(ref node) = tree.0 {
            match key.cmp(&node.key) {
                Less => tree = &node.left,
                Greater => tree = &node.right,
                Equal => return true,
            }
        }
        false
    }
}


#[cfg(test)]
mod tests{
    use std::fs::File;
    use std::io::Read;
    use test::Bencher;
    use crate::bucket::BST;
    use binary_search_tree::BinarySearchTree;
    #[test]
    fn idk(){
        let mut f = File::open("assets/keywords1.txt").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let s = &*s.as_bytes();
        let mut tree = BST::new();
        for i in s {
            tree.insert(i);
        }
        println!("{:?}", tree.0.unwrap());
    }
    #[bench]
    fn def_bst(b: &mut Bencher){
        let mut f = File::open("assets/keywords1.txt").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let mut tree = BinarySearchTree::new();
        let s = &*s.as_bytes();
        for i in s {
            tree.insert(i);
        }
        println!("{}",tree.size);
        b.iter(||{
            for i in s {
                assert!(tree.contains(&i));
            }
        });
    }
    #[bench]
    fn local_bst(b: &mut Bencher){
        let mut f = File::open("assets/keywords1.txt").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let s = &*s.as_bytes();
        let mut tree = BST::new();
        for i in s {
            tree.insert(i);
        }
        b.iter(||{
            for i in s {
                assert!(tree.contains(&i));
            }
        });
    }
}