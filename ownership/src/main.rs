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
    
    let _s2 = s;

    // When String that is allocated on the heap is created following is stored on the stack and is replicated when we try to bind it with another variable
    // [
    //     "pointer",
    //     "capacity",
    //     "length"
    // ]

    // println!("{}", s); // this line will generate an error
    //  The value of s is moved to s2 and s is no longer valid from this point
    // whenever a variable goes out of scope Rust calls a special function called "drop" similar to RAII(Resouce Acquisition Is Initialisation) pattern in C++

    let mut my_name: String = String::from("Subho Yadav");
    my_name = takes_ownership(my_name); // the ownership of my_name is transfered to the function takes_ownership

    println!("My name is: {}", my_name);

    let age: i32 = 24;
    makes_copy(age);


    // Returning a tuple from the function 
    let (my_name_after_ownership_returned, length) = calculates_length(my_name);

    println!("{} has a length of: {}", my_name_after_ownership_returned, length);
    
}

fn calculates_length(word: String) -> (String, usize) {
    let length = word.len();

    (word, length)
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    // This is very inconvinient to return whatever we recieve when we want to reuse the parameter in the parent function and to overcome this we have "references"
    some_string
}

fn makes_copy(some_value: i32) {
    println!("{} ", some_value);
}

// Any type that implements the 'copy' trait is copied and is not moved