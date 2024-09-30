// If something is not imported by default in the prelude, you have to import that into the scope of your program
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    loop {
        println!("Guess the number !");
        println!("Please input your guess: ");

        let mut guess = String::new();
        // creates a growable instance of 'String' provided by the standard library, which is a utf encoded bit or text

        // The :: represents that new is an associated function, which is a function that is implemented on a type

        // variables and references in rust are immutable by default

        let secret = rand::thread_rng().gen_range(1..=100);

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        // shadowing guess and changing its type
        // let guess: u32 = guess.trim().parse().expect("Please type a number !");

        // doing proper error handling while converting the string value 'guess' to a number
        let guess: u32 = match guess.trim().parse() {
            OK(num) => num,
            Err(_) => continue,
        }

        // 'read_line' returns a Result enumeration or enum which can have two variants, ok and Err

        println!("You have guessed: {}", guess);
        println!("secret number: {}", secret);

        // match statement is composed of arms
        match guess.cmp(&secret) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Tool small!"),
            Ordering::Equal => {
                println!("You win !");
                break;
            },
        }
    }

}
