use std::collections::HashMap;

fn main(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Red"), 100);
    scores.insert(String::from("Yellow"), 150);

    let yellow = match scores.get("Yellow"){
        Some(x) => x,
        None => &-1,
    };

    println!("yellow's value is {}", yellow);

    println!("The hashmap is {:?}", scores);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // let blue = match scores.get("Blue"){
    //     Some(x) => x, 
    //     None => &-1
    // };
    // println!("the value of the key blue is {}", blue);


    // scores.insert(String::from("Blue"), 25);
    // // this wil overwrite the current value in hashmap
    // println!("The hashmap is {:?}", scores);

}
