extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    loop {
        println!("Input Your Number.");
        let mut guessNum = String::new();

        io::stdin().read_line(&mut guessNum)
            .expect("Failed to read input");

        let guessNum: u32 = match guessNum.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("Your Guess Number is {}", guessNum);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }

}