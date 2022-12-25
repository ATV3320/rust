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
}
