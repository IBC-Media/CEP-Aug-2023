struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Marks<T, U>{
    a: T,
    b: U,
}

impl<T, U> Point<T, U> {
    fn mixup<T, U>(self, other: Point<T, U>) -> Point<T, U> {
        Point {
            a: self.a,
            b: other.b,
        }
    }
}

fn main() {
    // this allows us to use the same struct to have different types
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let character = Point { x: 'a', y: 'b' };

    let m1 = Marks{a: 25, b: 35.0};
    let m2 = Marks{a: 34, b:56};


    // methods
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}