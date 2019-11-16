#[derive(Debug)]
enum Shape {
    Rectangle { width: f64, height: f64 },
    Square(f64),
    Circle(f64),
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Rectangle { width, height } => width * height,
            Shape::Square(size) => size * size,
            Shape::Circle(radius) => std::f64::consts::PI * (radius * radius),
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}

fn main() {
    let shape1 = Shape::Rectangle { width: 30.0, height: 50.0 };
    let shape2 = Shape::Square(10.0);
    let shape3 = Shape::Circle(5.0);
    let shape4 = Shape::Triangle { base: 20.0, height: 100.0 };
    
    println!("shape1 is {:?}", shape1);
    println!("shape2 is {:?}", shape2);
    println!("shape3 is {:?}", shape3);
    println!("shape4 is {:?}", shape4);
    println!("The area of the shape1 is {} square pixels.", shape1.area());
    println!("The area of the shape2 is {} square pixels.", shape2.area());
    println!("The area of the shape3 is {} square pixels.", shape3.area());
    println!("The area of the shape4 is {} square pixels.", shape4.area());
}