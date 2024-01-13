fn main() {
    let mut x: i32 = 5;
    println!("The value of x is {}", x);
    const MY_VARIABLE: u32 = 555_000;
    x = 6;
    println!("The value of x now is: {}", x);
    //constants can not be set to return values of a function
    println!("Our constant, not variable is: {}", MY_VARIABLE);
    let y: i32 = 54;
    let y:i32 = 55;
    //for y, this is shadowing, re-declaring the variable.

    // const MY_VARIABLE: u32 = 111_000;
    // We can not do shadowing with constants as it will otherwise be redeclared... contradicting its properties

    //Int, float, bool, char
    let about_to_overflow: u8 = 255;
    // in debug builds, the code will panic, but in release builds, we will simply do the modulo thing, 2s compliment
    //mapping
    let g = 2.0;
    //by default, floats are of type f64, but we can do f32 as well

    let char1 = 'a';
    //compound data types

    let tuple1 = ("Aryan", "2k18se039", 101_000);
    //for releated data types, maybe we will encounter more of these when dealing with databases
    let (name, roll_no, bal) = tuple1;
    let (name1, _, _) = tuple1; //I can do this when I only need one.
    let mut name2 = tuple1.0; //or this
    name2 = "pina colada";
    println!("And printing name is prolly not possible {}", name);
    println!("Printing name1: {}", name1);
    println!("Printing name2 as well, {}", name2);
    //this is call by reference? Nope, they are cloning.
    let arr1:[f32; 3] = [525.0, -232.3, 43.1];
    for (index, number) in arr1.iter().enumerate() {
        println!("There's {} in the array, it is the {} element", number, index);
    }
    //an even cooler way of initializing an array is this:
    let byte = [0;20];
    //it makes an array of 20 elements, all initialized to zero
    // println!("I'll try printing the 21st element of byte array{}", byte[21]); //throws an error- out of bounds

    //now functions
    //statements perform an action and expressions return values, a pt

    //for control flows, in rust, the condition must explicitly be a boolean
    let my_special_variable = if x>y {"more"} else {"less"};
    println!("We can see my_special_variable being populated after going through a condition: {}", my_special_variable);
    let mut counter = 0;
    const COMPARATOR: i32 = 6;
    loop{
        println!("printing x:{} many times", counter);
        counter = counter + 2;
        if counter >= COMPARATOR{
            break;
        }
    }
    //while and for loops are normal
    //we also have capability of going through the range
    for number in 1..3{ //works like [)
        println!("{}!", number);
    }

}
