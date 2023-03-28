use std::io;

fn main() {
   let mut values: (u128, u128) = (0, 1);

    println!("how high a fib you want?");

    let mut fib_top = String::new();

    io::stdin()
        .read_line(&mut fib_top)
        .expect("please enter a number");


    let fib_top: u128 = match fib_top.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("you didn't enter a number"),
    };
    

   loop {
    let (x, y) = values;
    if y < fib_top {
        values = (y, x + y);
        println!("fib num: {x}")
    } else {
        println!("final nums: {x} {y}");
        break;
    }
   }
}
