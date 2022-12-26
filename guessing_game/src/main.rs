// use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    print!("Guess the number, let's play..\n");

    let secret_number = rand::thread_rng().gen_range(1, 11);

    loop {
        println!("Input a number bw 1 and 10:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!(
            "The number guess is {} but the random number however is: {}",
            guess, secret_number
        );
        // match guess.cmp(&secret_number) {
        //     Ordering::Less => println!("{}", "Too small".red()),
        //     Ordering::Greater => println!("{}", "Too big".red()),
        //     Ordering::Equal => {
        //         println!("{}", "You win.".green());
        //         break;
        //     }
        // }
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small"),
            Ordering::Greater => println!("{}", "Too big"),
            Ordering::Equal => {
                println!("{}", "You win.");
                break;
            }
        }
    }
}
