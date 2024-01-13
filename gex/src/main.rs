// mod datatype;
// mod ownership;

fn main() {
    println!("Ownership.rs");

    //calloc and malloc in languages like C
    //1 Each value in rust has a variable that's called its owner
    //2 THere can only e one owner at a time.
    //3 When the owner goes out of scope, the value will be dropped.

    //there are string literals and there is the string type.
    //if we use string literals it will be converted to binary, we won't be able to change the size

    let s1: &str = "Hello";
    let s2: String = String::from("Hello");
    let x = s1;
    let y = s2;

    println!("The value of parent s1 is {}, and value of borrower x is {}", s1, x);
    // println!("The value of parent s2 is {}, and the value of borrowed y is {}", s2, y);
    // the above statement doesn't work, as the String type will only remove s1 and move the ownership of
    //value to y.
    
}
