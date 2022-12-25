fn main() {
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
        r = r-5;
        let s = s-5;
        println!("check 2, value of r is {}", r);
        println!("check 2, value of s is {}", s);
    }
    r = r + 5;
    s = s + 5;
    println!("check 3, value of r is {}", r);
    println!("check 3, value of s is {}", s);
    //so we can say that in this case, only one reference of r is accessed at both times,
    //but s was accessed differently.


    const SECONDS_IN_A_MINUTE:i32 = 60;
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
}
