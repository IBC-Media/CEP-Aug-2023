struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
// debug trait is required to print a struct
#[derive(Debug)]
struct Color(i32, i32, i32);
fn main() {
    // creating an object for the struct User 
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    // we change the email for user 1
    user1.email = String::from("anotheremail@example.com");
    // autofill data from another object
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // tuple struct
    let black = Color(0, 0, 0);
    println!("{:?}", black);
}
