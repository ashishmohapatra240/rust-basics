struct Rect<T> {
    width: T,
    height: T,
}

impl <T: std::ops::Mul<Output = T> + Copy> Rect<T> {
    fn area(&self) -> T {
        return self.width * self.height;
    }
}

fn main() {
    let r = Rect {
        width: 10,
        height: 20,
    };
    let r1 =Rect{
        width: 10.0,
        height: 20.0,
    };
    println!("{}", r.area());
    println!("{}", r1.area());
}
