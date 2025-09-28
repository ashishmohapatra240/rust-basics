struct Rect {
    height: f32,
    width: f32,
}

impl Rect {
    fn area(&self) -> f32 {
        return self.height * self.width;
    }
    fn print_something(){
        println!("Something");
    }
}

fn main() {
    let r = Rect {
        width: 10.0,
        height: 20.3,
    };

    println!("{}", r.height);
    println!("{}", r.width);

    println!("{}", r.area());
    Rect::print_something();
}
