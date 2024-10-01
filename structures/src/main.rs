struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn build_user(username: String, email: String) -> User {
    // field inint shorthand where the parameter name and the field name are the same and we do not need to repeat them
    User {
        username,
        active: true,
        email,
        sign_in_count: 1
    }
}
fn main() {
    let mut user1 = User {
        active: true,
        email: String::from("subhoyadav16@gmail.com"),
        username: String::from("Subhoi Yadav"),
        sign_in_count: 1
    };

    // If the instance of the strict is mutable as a whole we can change the value of the fields in the struct
    println!("email: {}", user1.email);
    user1.email = String::from("subhoyadav79@gide.ai");
    println!("email: {}", user1.email);

    user1.email = String::from("subhoyadav79@gide.ai");

    let mut user2: User = build_user("Amana Bansal".to_string(), "aman@gmail.com".to_string());
    println!("Aman's details {}", user2.username);
    // The keys of a struct are called field 

    let mut user3 = User {
        email: String::from("aman+user3@gmail.com"),
        ..user2
    };

    // When we use the struct update syntax it performs a similar operation as the assignment operator so the data types which are not created on the stack are moved and eventualy user2 loses ownership of the username field

    // struct update syntax should be the last statement when u create a struct

    println!("user3 email {}", user3.email);
    println!("user2 email {}", user2.email);

    // println!("user2 username {}", user2.username);
    
    // Tuple structs
    struct Color(i32, i32, i32);

    struct Corordinate(i32, i32, i32);

    let mut black = Color(255, 255, 255);
    let mut origin = Corordinate(0,0,0);

    println!("origin {}, {}, {}", origin.0, origin.1, origin.2);
    
    // unit-like structs
    struct AlwaysEqual;

    let subject = AlwaysEqual;

    // unit-like structs comes in handy when we want to implement a trait on the type but do not have any data to put in it



}
