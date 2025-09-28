use std::f32::consts::PI;

enum Shape {
    Circle(f32),
    Rectangle(f32, f32),
    Square(f32),
}
fn main() {
    let shape: Shape = Shape::Square(10.0);
    let shape_circle: Shape = Shape::Circle(10.0);
    let shape_rectangle: Shape = Shape::Rectangle(10.0, 20.0);

    println!("{}", area(shape));
    println!("{}", area(shape_circle));
    println!("{}", area(shape_rectangle));
}

fn area(shape: Shape) -> f32 {
     return match shape {
        Shape::Circle(radius) => radius * radius * PI,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side,
    };
}
