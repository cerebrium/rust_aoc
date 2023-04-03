use std::io;

fn main() {
    println!("Please input a number for fizzBuzz");

    let mut inputForFizz = String::new(); 

    io::stdin()
        .read_line(&mut inputForFizz)
        .expect("Failed to read line");

    let fizzAmount: usize = match inputForFizz.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("cannot convert input to usize"),
    };

    let fizzBuzzArr = fizzBuzz(fizzAmount);
    println!("Result: {:?}", fizzBuzzArr);
    
}

fn fizzBuzz(n: usize) {
    let mut result = vec![""; n];

   for number in 1..n {
    // push the vlaue of the fizzBuzzMatcher
    // into the result vector

    result.push(&fizzBuzzMatcher(number));
   } 

    result;
}

/*


    This needs to return a string 
    that was formatted by the rules of
    fizzBuzz.

 */

fn fizzBuzzMatcher(n: usize) -> String {
    let mut resultantStr = String::new();

    match n {
        n if (n%3 == 0) => resultantStr.push_str("fizz"),
       n if (n%5 == 0) =>  resultantStr.push_str("buzz"), 
       _ =>  resultantStr.push_str(&n.to_string()),

    };

    resultantStr
}

