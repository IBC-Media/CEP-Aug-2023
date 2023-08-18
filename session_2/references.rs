fn main(){
    let s1 = String::from("hello");
    display_string(&s1);
    println!("s1 is {s1}");


    // let's examin mutable references, thos which enables us to edit the real value

    let mut s = String::from("Hello ");
    add_word(&mut s);
    // here we can see that the reference can be used to modify the actual data
    println!("s is {s}");

    // rust doesnt allow us to have multiple mutable references at the same time
    // also we cannot have a mutable reference be used before completely using all the immutable refernces
    // this code works very well

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);


    // Dangling references
    // this refernce would give an error as it tries to return a reference to a string which only lives inside the function
    let reference_to_nothing = dangle();

}
fn display_string(s: &String){
    println!("The input string is {s}");
}

fn add_word(s: &mut String){
    s.push_str("World!");
}
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}