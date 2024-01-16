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
    let x = s1; //copy
    let y = s2; //basically data from s1 has not been copied, it has been moved to y
    let z = y.clone(); //this is clone
    println!("The value of parent s1 is {}, and value of borrower x is {}", s1, x);
    // println!("The value of parent s2 is {}, and the value of borrowed y is {}", s2, y);
    // the above statement doesn't work, as the String type will only remove s1 and move the ownership of
    //value to y.

    //as we pass a value to a function => it is the same as we assign that value to another variable.
    //but if the value is small?? Like a character or integer or bool, it will simply be copied
    let p1: String = gives_ownership();
    let p2: String = String::from("About to be shaken");
    let mut p3 = takes_ownership_and_returns_back(p2);

    //but say we don't want to remove the old p2... we want it to be used later as well
    //in this case, we will pass-by-reference
    let p1_length = calculate_length(&p1);
    let p3_length = calculate_length(&p3);
    println!("Size of p1 and p3 are {} and {} respectively", p1_length, p3_length);
    modify_by_reference(&mut p3);
    println!("Printing the modified p3 after mod function's scope ends, {}", p3);

    //One more important thing:
    //Scope of a variable starts when it is first introduced,
    //and ends when we use it for the last time... does that mean we can actually have two mutable references,
    //one after another? Let's see
    let mut solid_pillar = String::from("Solid_Pillar");
    let mut sol_pil = "sol_pil";

    //Trial of having two mutable references with type &str
    // let mut r1 = &mut sol_pil;
    // *r1 = "sola";
    // let mut r2 = &mut sol_pil;
    // println!("Sol pil can have two mutable references? {}, {}", r1, r2);

    let r11 = &solid_pillar;
    let r22 = &solid_pillar;
    println!("Our r11 and r22 respectively are: {} and {}", r11, r22);
    //this print line.. so basically since there is a mutable reference to my variable,
    //I can't use it at that time, even with its immutable references

    let r33 = &mut solid_pillar;

}
fn takes_ownership(some_string: String) {
    println!("The string is: {}", some_string);
}

fn gives_ownership()->String{
    String::from("Gifted string returned back")
}

fn takes_ownership_and_returns_back(some_string: String) -> String {
    println!("Our string is returned backk!");
    some_string
}

fn makes_copy(some_integer: i32) {
    println!("The copied integer is: {}", some_integer);
    //maybe make a generic type function that basically takes in values that will be copied..
    //will we be able to distinguish for this? Or will it be user's responsibility to call
    //the function that suits the requirements?
}

fn calculate_length(s: &String) -> usize {
    //this here, passing in references as function params is called borrowing
    //references are immutable by default
    s.len()
}

fn modify_by_reference(s: &mut String) {
    s.push_str(" , you've been mutated");
    println!("The string now is: {}", s);
}
