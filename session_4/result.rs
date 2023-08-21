use std::fs::File;
use std::io::{self, Read};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    // ok is returned when everything goes fine
    // err when there is an error
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    // using the expect
    let greeting_file = File::open("hello.txt")
    .expect("hello.txt should be included in this project");

    // the ? operator

}
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
//  the above function can again be simplified 
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
