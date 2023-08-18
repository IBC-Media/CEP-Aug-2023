#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        3.14*self.radius*self.radius
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let c1 = Circle {
        radius: 25.0,
    };

    println!("The area of the circle is {}", c1.area());
}