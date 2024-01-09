use std::io::{stdin, self, Write};

fn main() {
    let mut my_string = String::new();
    print!("Enter a number: ");

    io::stdout().flush().unwrap();

    let my_num = loop {
        my_string.clear();
        stdin().read_line(&mut my_string)
        .expect("did not enter a valid string.");

    match my_string.trim().parse::<f64>() {
        Ok(_s) => break _s,
        Err(_err) => println!("try again. Enter a number.")
    }
    };
    println!("you entered {:?}", my_num);

}