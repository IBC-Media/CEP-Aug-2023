fn add(x:i32, y:i32) -> i32{
    x+y
}

fn sub(x:i32, y:i32) -> i32{
    x-y
}

fn mul(x:i32, y:i32) -> i32{
    x*y
}

fn div(x:i32, y:i32) -> i32{
    x/y
}
fn main(){
    let a = 5;
    let b = 3;
    println!("{a} + {b} = {}", add(a, b));
    println!("{a} - {b} = {}", sub(a, b));
    println!("{a} * {b} = {}", mul(a, b));
    println!("{a} / {b} = {}", div(a, b));
}