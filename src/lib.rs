pub mod shape {
    #[derive(Debug)]
    pub enum Shape {
        Rectangle { width: f64, height: f64 },
        Square(f64),
        Circle(f64),
        Triangle { base: f64, height: f64 },
    }

    impl Shape {
        pub fn area(&self) -> f64 {
            match self {
                Shape::Rectangle { width, height } => width * height,
                Shape::Square(size) => size * size,
                Shape::Circle(radius) => std::f64::consts::PI * (radius * radius),
                Shape::Triangle { base, height } => 0.5 * base * height,
            }
        }
    }
}