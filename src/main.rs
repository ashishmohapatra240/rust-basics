struct User{
    username: String,
}

fn main() {
    let u =User{
        username: String::from("Ashish"),
    };
    print_variable(1);
    print_variable(1.1);
    print_variable(true);
    print_variable(String::from("Ashish"));
 
}

fn print_variable<T: std::fmt::Display>(a: T){
    println!("{}", a);
}