// x and y are parameters 
fn add(x:i32, y:i32)->i32{
    x+y
    // return x + y
}
fn main(){
    let a = 1;
    let b = 2;
    let c = add(a, b);
    println!("c is {}", c);
}