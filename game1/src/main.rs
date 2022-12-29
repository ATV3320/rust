extern crate chrono;
use std::io;
use chrono::prelude::*;

fn main() {
    println!("I will tell you when you would turn hundred\n");
    let name = input("Hello, what is your name? ").expect("Can't process name, some error occured!!");
    println!("Hello, {}!, I am Rust", name);
    let age = input("So friend, what is your age? ").expect("Can't process age, something went wrong!!")
    .parse::<i32>().expect("Invalid age, are you sure you input a number?");
    println!("Hmm, so you are {} years old.", age);
    calculateWhen100(&name, &age);
}

fn input(user_message: &str) -> io::Result<String> {
    use std::io::Write;
    print!("{}", user_message);
    io::stdout().flush()?;

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim_end().to_owned())
}

fn calculateWhen100(user_name: &String, user_age: &i32) {
    //-> io::Result<i32> 
    //current date
   let current_year = Utc::now().year();
   let hundred_year = 100 - user_age + current_year;
   println!("Hello {}, you will turn 100 in year {}", user_name, hundred_year);
}