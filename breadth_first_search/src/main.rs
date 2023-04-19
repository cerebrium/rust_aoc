
mod Binary_DS {

   #[derive(Debug)]
   struct BNode<T> {
       value: u32,
       left: Box<BNode<T>>,
       right: Box<BNode<T>>,
   } 
}

fn main() {
    println!("Hello, world!");
    for number in (0..10) {
        println!("number is: {}", number);
    }
}
