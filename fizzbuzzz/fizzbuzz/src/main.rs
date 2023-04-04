use std::io;

fn main() {
    println!("Please input a number for fizzBuzz");

    let mut input_for_fizz = String::new(); 

    io::stdin()
        .read_line(&mut input_for_fizz)
        .expect("Failed to read line");

    let fizz_amount: usize = match input_for_fizz.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("cannot convert input to usize"),
    };

    let fizz_buzz_arr = fizz_buzz(fizz_amount);
    println!("Result: {:?}", fizz_buzz_arr);
    
}

fn fizz_buzz(n: usize) -> Vec<String> {
    let mut result = Vec::new();


   for number in 1..=n {
    // push the vlaue of the fizzBuzzMatcher
    // into the result vector
    // result[number - 1] = &fizz_buzz_matcher(number);
    let res = fizz_buzz_matcher(number);
    result.push(res)
   }

   result
}

/*


    This needs to return a string 
    that was formatted by the rules of
    fizzBuzz.

 */

fn fizz_buzz_matcher(n: usize) -> String {
    let mut resultant_str = String::new();

    match n {
        n if (n%3 == 0) => {
            match n {
                n if (n%5 == 0) => resultant_str.push_str("fizzbuzz"),
                _ => resultant_str.push_str("fizz")
            }
        },
       n if (n%5 == 0) =>  resultant_str.push_str("buzz"), 
       _ =>  resultant_str.push_str(&n.to_string()),

    };

   resultant_str 
}

