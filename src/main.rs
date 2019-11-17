mod shape;
use shape::Shape;

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