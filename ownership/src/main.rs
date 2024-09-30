fn main() {

    // Rules of ownership
    // 1. Every value in rust has a owner
    // 2. When the owner goes out of scope, the value will be dropped
    // 3. There can be only one owner to a value at any time

    // string literals are immutable 
    let str_literal = "Subho Yadav";
    println!("{str_literal}");

    // Creating a String type out of a string literal
    let mut s = String::from("Hello");
    s.push_str(" World");
    println!("{s}");
    
    let s2 = s;

    println!("{}", s); // this line will generate an error
    //  The value of s is moved to s2 and s is no longer valid from this point
    // whenever a variable goes out of scope Rust calls a special function called "drop" similar to RAII(Resouce Acquisition Is Initialisation) pattern in C++
}
