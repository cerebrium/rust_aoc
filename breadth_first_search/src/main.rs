
mod binary_ds {
   #[derive(Debug)] 
   pub struct BNode {
       pub value: i32,
       pub left: Option<Box<BNode>>,
       pub right: Option<Box<BNode>>,
   } 

   impl BNode {
      pub fn create_b_node(val: i32) -> BNode {
        BNode {
            value: val,
            left: None,
            right: None 
        }
      }

      pub fn insert_node(&mut self, value: i32) {
          // Wrap it in an option so that it won't throw
          // if the creation process fails
          let new_node = Some(Box::new(BNode::create_b_node(value)));
          if value < self.value {
              // If there is a current value that is less
              // then call again the method implementation
              // to go further down the tree;
            match self.left.as_mut() {
                None => self.left = new_node,
                Some(left) => left.insert_node(value)
            }

          } else {
              match self.right.as_mut() {
                  None => self.right = new_node,
                  Some(right) => right.insert_node(value)
              }
          }
      }
   }
}

use crate::binary_ds::BNode; 

fn main() {
    println!("Hello, world!");
    let mut parent = BNode::create_b_node(10); 
    for num in 1..25 {
        BNode::insert_node(&mut parent, num);
    }
    println!("the node{:?}", parent)
}
