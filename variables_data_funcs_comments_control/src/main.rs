use std::io;

fn main() {
   let mut values: [u128; 2] = [0,1];

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
       let x = values[0];
       let y = values[1]; 
    if y < fib_top {
        values[0] = y;
        values[1] = x + y;
        if values[1] < fib_top {
        print_values(&values[1]);
        }
    } else {
        break;
    }
   }
}

fn print_values(num: &u128) {
    println!("Fib Num: {num}")
}
