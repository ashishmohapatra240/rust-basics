struct Rect{
    width: f32,
    height: f32,
}

impl Shape for Rect{
    fn area(&self) ->f32{
        return self.width * self.height;
    }

}
struct Circle{
    radius: f32,
}


impl Shape for Circle{
    fn area(&self) ->f32{
        return self.radius * self.radius * 3.14;
    }
}

trait Shape{
    fn area(&self) ->f32;
}


fn print_area_of_shape<T: Shape>(shape: T){
    println!("{}", shape.area());
}

fn main() {
    let r = Rect{width: 10.0, height: 20.0};
    let c = Circle{radius: 10.0};
    print_area_of_shape(r);
    print_area_of_shape(c);

}
