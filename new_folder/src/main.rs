use core::num;
pub mod trialFile1;
use std::io;

fn main() {
    trialFile1::main();
    println!("Really sorry that I'm doing this still.");
    let mut p = 5;
    println!("Print the value of p: {}", p);
    p = 17;
    println!("Changed value of p is: {}", p);

    let q = 2;
    println!("Value of q is: {}", q);

    let q = "parh";
    println!("Changed value of q is {}", q);

    // a good lesson here,:
    {
        let q = 4;
        println!("In the scope q is: {}", q);
    }
    // q = q +1;
    // throws error since outside of the scope, q is still a string.
    println!("I am {}", q);

    let mut r = 10;
    let mut s = 10;
    println!("check 1, value of r is {}", r);
    println!("check 1, value of s is {}", s);

    {
        r = r - 5;
        let s = s - 5;
        println!("check 2, value of r is {}", r);
        println!("check 2, value of s is {}", s);
    }
    r = r + 5;
    s = s + 5;
    println!("check 3, value of r is {}", r);
    println!("check 3, value of s is {}", s);
    //so we can say that in this case, only one reference of r is accessed at both times,
    //but s was accessed differently.

    const SECONDS_IN_A_MINUTE: i32 = 60;
    println!("value of seconds is {}", SECONDS_IN_A_MINUTE);

    let tup: (i32, bool, char) = (1, true, 'a');
    println!("{}", tup.1);

    let mut tup1: (i32, bool, char) = (1, true, 'a');
    println!("{}", tup1.1);
    tup1.1 = false;
    println!("{}", tup1.1);

    //arrays are crazier tho,
    let arr0 = [1, 2, 3];
    println!("{}", arr0[1]);
    //we have to make them mutable in order to change the elements inside,
    // character of the array is the type of datatype in it and the size.
    let mut arr1: [i64; 5];
    // println!("{}", arr1[1]);
    //gives error, there is no automatic initialization.
    //even if i equate it to an empty array.
    arr1 = [2, 3, 4, 5, 6];
    println!("{}", arr1[4]);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("Will hit it with a {}", input);
    //we will have to put String::new() whenever we want to input something,  (in case of read_line)

    //arithmetic
    let x: u8 = 55;
    let y: u8 = 10;
    // let mut z = x + y; no implementation for adding i8 and u8
    // println!("Adding those different x and y: {}", z);
    //it also handles overflow pretty well here..
    let mut z = x / y;
    println!("{}", z);

    let p: i64 = 125_000;
    let q: i32 = 500;
    //now I wanna typecast.

    let mut r = p / (q as i64);
    println!("{}", r);

    //so, rust does not automatically convert types.

    //like we talked above, what if we want to take an integer as input, how would we convert a string to integer?
    //this is how..
    let mut integer1 = String::new();
    io::stdin()
        .read_line(&mut integer1)
        .expect("Expect to read line");

    let int_input: i64 = integer1.trim().parse().unwrap();
    //trim removes invisible characters, parse checks for integer, unwrap converts that data into integer.

    println!("{}", int_input + 20);

    //condionals

    let cond1 = (2 as f32) <= 2.2;
    println!("cond1 is {}", cond1);

    let cond2 = false && cond1; //yeah that && and,  || or
    println!("cond2 is {}", cond2);

    //how do we use them? this is how:
    if cond1 {
        println!("cond1 was true, so I got printed.");
    }

    if cond2 {
        println!("cond2 was true, so I got printed.");
    } else {
        println!("cond2 was false, so I got printed.\n");
    }

    //they also have weirder syntax, see:

    let name = "Aryan";

    if name == "Vidyut" {
        print!("Yes, I knew it, that the name would be Vidyut.");
    } else if name == "Aryan" {
        println!("Ok, so it is Aryan,");
    } else if name == "Tejas" {
        println!("Ok, so it is Tejas,");
    } else {
        println!("Well, who would have guessed.")
    }

    //calling the function
    test_one();
    let nu = test_two();
    println!("in test_two, I am the value of the number, {}", nu);

}


//yippie, functions finally.

fn test_one() {
    println!("I am from function test_one, I just got called.");
}

//in rust, there is clear distinction bw statements and expressions.

fn test_two()->i32 {
    let mut number = {
        let mut x = 3;
        x+1
    };
    // this above, the x+1, without the semicolon, that is an expression...
    println!("{}", number);
    return number;
}
//we need to have an expression at the end of the function.eeEEEEEX