fn main(){
    // creating a vector two methods
    // 1. using Vec::new() function
    let v: Vec<i32> = Vec::new();

    // 2. using the vec! macro
    let v = vec![1, 2, 3];


    // inserting values into the vector
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // accessing elements from a vec
    println!("The 2nd element is {}", v[1]);
    let thrid_element = v.get(10);

    match thrid_element {
        Some(x) => println!("The third element is {}", x),
        None => println!("Invalid Index")
    }

    // iterating through all of the elements in a vec

    for i in v{
        print!("{i} ");
    }
}