enum IpAddr {
    V4(String),
    V6(u8, u8, u8, u8, u8, u8),
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

fn main(){
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
    let home = IpAddr::V4(String::from("127.0.0.1"));

    // the option enum
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    // using option and match
    let five = Some(5);
    let six = plus_one(five);
    println!("{six}");
    let none = plus_one(None);
}
fn plus_one(x: Option<i32>) -> i32 {
    // match statements
    match x {
        None => -1,
        Some(i) => i + 1,
    }
}