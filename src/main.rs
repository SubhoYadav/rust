// Install rust on linux and macos: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
// starting a new rust project: cargo new your-project
fn main () {
    // prevents the warning of unused variables
    // #[allow(unused_variables)]

    let name: &str = "Subho Yadav";
    println!("Hi my name is: {}", name);

    // can delare a variable with the same name in rust to shadow it
    let name: &str = "Aman Bansal";
    println!("My name is {}", name);

    // prefix a variable name with '_' to prevent the warning of unused variables
    let _k: i32 = 10;

    // Destructuring in rust
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("The value of x is {} and y is {}", x, y);

    (3,2); // tuple
    [2,3]; // array

    let (x,y);
    [x, ..] = [3, 2];
    [.., y] = [3, 2];

    assert_eq!([x, y], [3, 2]);
    
    // integer types: i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, i-signed and u-unsigned
    // the numbers are the bits in the integer 

    // Type casting in rust using the 'as' keyword
    let _k: u16 = 38_u8 as u16;
    println!("Hey we reached the end!");

    "u32".to_string(); // creates a new string instance

    println!("{}",i8::MAX); // constants
    println!("{}", u8::MAX); // constants
    
    let result = i8::checked_add(10, 245).unwrap();
    // checked_add returns None if the addition overf   lows i8 and Some(result) if it does not, also unwrap is used to extract the value from Some(result)

    println!("{}", result);

}