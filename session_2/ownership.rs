fn main(){
    // move in case of integers
    let x = 5;
    let y = x;
    println!("x is {x} and y is {y}");

    // in case of string
    let s1 = String::from("hello");
    let s2 = s1;
    // trying to access s1 will result in a error as s1 is already moved to s2
    // println!("s1 is {s1} and s2 is {s2}");


    // how to solve the issue ?
    // so we can rather prevent rust from taking a deep copy
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 is {s1} and s2 is {s2}");

    // the classical case of passing into a function
    // so what happens when we pass a string into a function that doesn't return anything

    let s1 = String::from("hello");
    // here s1 is moved into the function and hence it will dropped once the function call is completed

    display_string(s1);
    // tring to acces s1 will return a error
    println!("s1 is {s1}");

    // but this kind of a problem can be solved via different approaches
    // 1.taking a clone of s1 and passing it
    // 2. returning the string back
    // 3. pass the refernce of the string

    // lets examine the 3rd case
}

fn display_string(s:String){
    println!("The input string is {s}");
}
