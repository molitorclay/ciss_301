/*
    Clay Molitor
    Lab 3 - Recursion, Series, Irrational Numbers

    My program can calculate Factorials, Fibanacci, and PI for user input values.
    See lab report for detailed writeup.
*/
use num_bigint::BigUint;
use num_bigint::BigInt;
use std::io;


fn main() {
    user_input();
}

fn user_input () {

    let mut input_text = String::new();

    loop {
        println!("
Please choose a function to run by entering a number 1-3, or Q to exit.
1) Factorial
2) Fibinacci
3) π");

        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed_input = input_text.trim();
        match &(trimmed_input) {
            &"1" => { // Factorial
                println!("Please enter a value to Factorialize: ");
                println!("\tFactorial: {}", factorial(number_from_console()));
            },
            &"2" => { // Fibonacci
                println!("Please enter a degree of Fibinacci to calculate: ");
                println!("\tFibonacci: {}", fibonacci(number_from_console()));
            },
            &"3" => { // Pi
                println!("Please enter the starting range of pi to calculate: ");
                let a = number_from_console();
                println!("Please enter the ending range of pi to calculate: ");
                let b = number_from_console();
                match a {   // Calculate pie to the end range and trim off begining
                    0 => println!("\tPI: 3.{}", &calc_pi(b)[a..]), // Print "3." if starting from 0
                    _ => println!("\tPI: {}", &calc_pi(b)[a..]), 
                }
            },
            &"Q" | &"q" => { 
                println!("Goodbye! 👋");
                break;
            },
            _ => { 
                println!(" I'm sorry, I don't understand [{trimmed_input}], please try again.")
            },
        }
        input_text.clear()
    }
}
// Returns an int read from the console.
// Loops until an int is read, prompting for reentry to console.
// Mutation warning, reads from console.
fn number_from_console() -> usize
{
    let mut input_text = String::new();
    loop {
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        match input_text.trim().parse::<usize>() {
            Ok(i) => {  return i; },
            Err(..) => println!("this was not an integer: [{}], please try again: ", input_text.trim()),
        };
        input_text.clear()
    }

}
// ------------------------------------ Math Functions ------------------------------------

// Calculates factorial of n
// Result can be arbitrarily large.
fn factorial (n: usize) -> BigUint
{       
    let mut result = BigUint::new(vec!(1)); 
    let mut i = n;

    while i > 1
    {
        result = result * i;
        i -=1;
    }
    result
}
// Calculates fibinacci number at n
// Result can be arbitrarily large.
fn fibonacci (n: usize) -> BigUint
{       
    let mut prever  = BigUint::new(vec!(1)); // n - 2
    let mut prev    = BigUint::new(vec!(0)); // n - 1
    for _ in 0..n
    {
        let result = &prever + &prev;
        prever = prev;
        prev = result;
    }
    return prev;
}

/*
    Spigot formula from https://rosettacode.org/wiki/Pi#Rust
    My only modification to this function is to pass an int for target length and return a string.
    Returned digits start after the decimal point, ex: 1459...
*/
fn calc_pi(length: usize) -> String {

    let mut result = String::new();
    let mut q = BigInt::from(1);
    let mut r = BigInt::from(0);
    let mut t = BigInt::from(1);
    let mut k = BigInt::from(1);
    let mut n = BigInt::from(3);
    let mut l = BigInt::from(3);
    let mut first = true;
    while result.len() < length {
        if &q * 4 + &r - &t < &n * &t {
            // Don't print the 3
            if !first { 
                result.push(format!("{n}").chars().next().unwrap());
            }
            else {
                first = false;
            }
            let nr = (&r - &n * &t) * 10;
            n = (&q * 3 + &r) * 10 / &t - &n * 10;
            q *= 10;
            r = nr;
        } else {
            let nr = (&q * 2 + &r) * &l;
            let nn = (&q * &k * 7 + 2 + &r * &l) / (&t * &l);
            q *= &k;
            t *= &l;
            l += 2;
            k += 1;
            n = nn;
            r = nr;
        }
    }
    return result;
}