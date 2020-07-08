#![allow(dead_code)]
use std::cmp::Ordering::{Equal, Greater, Less};
// use std::mem;

#[derive(Debug)]
pub struct BST {
  root: Link,
}

#[derive(Debug)]
enum Link {
  Empty,
  More(Box<Node>),
}

#[derive(Debug)]
struct Node {
  elem: i32,
  left: Link,
  right: Link,
}

fn main() {
  impl BST {
    pub fn new() -> Self {
      BST { root: Link::Empty }
    }

    // Insert an element into the BST. Return true if successful, or false if the element was already in the BST.
    pub fn insert(&mut self, elem: i32) -> bool {
      match self.search(elem) {
        true => false,
        _ => self.root.insert(elem),
      }
    }

    // Search for an element in the BST. Return true if the element was found.
    pub fn search(&mut self, val: i32) -> bool {
      self.root.search(val)
    }
  }

  impl Link {
    fn insert(&mut self, elem: i32) -> bool {
      match self {
        Link::Empty => {
          let new_node = Box::new(Node {
            elem,
            left: Link::Empty,
            right: Link::Empty,
          });
          *self = Link::More(new_node);
          true
        }
        Link::More(node) => match elem.cmp(&node.elem) {
          Equal => false,
          Less => node.left.insert(elem),
          Greater => node.right.insert(elem),
        },
      }
    }

    fn search(&self, elem: i32) -> bool {
      match self {
        Link::Empty => false,
        Link::More(node) => match elem.cmp(&node.elem) {
          Equal => true,
          Less => node.left.search(elem),
          Greater => node.right.search(elem),
        },
      }
    }
  }

  let mut bst = BST::new();

  println!("{:?}", bst);
  bst.insert(1);
  bst.insert(2);
  bst.insert(2);
  println!("{:?}", bst);
}

#[cfg(test)]
mod test {
  use super::BST;

  #[test]
  fn create_bst() {
    let mut bst = BST::new();
    assert_eq!(true, bst.insert(1));
    println!("{:?}", bst);
  }

  #[test]
  fn insert_duplicates() {
    let mut bst = BST::new();
    bst.insert(11);
    bst.insert(11);
    bst.insert(11);
    println!("{:?}", bst);
  }

  #[test]
  fn insert_diff() {
    let mut bst = BST::new();
    assert_eq!(true, bst.insert(0));
    assert_eq!(true, bst.insert(-1));
    assert_eq!(true, bst.insert(-2));
    println!("{:?}", bst);
  }

  #[test]
  fn search_inserted() {
    let mut bst = BST::new();
    bst.insert(1);
    bst.insert(2);
    bst.insert(3);

    assert_eq!(true, bst.search(2));
    println!("{:?}", bst);
  }

  #[test]
  fn search_other() {
    let mut bst = BST::new();
    bst.insert(1);
    bst.insert(2);
    bst.insert(3);

    assert_eq!(false, bst.search(5));
    println!("{:?}", bst);
  }
}
