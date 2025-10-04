struct User<'a> {
    username: &'a str,
    password: &'a str,
}
fn main() {
    let str1 = String::from("Ashish");
    let str2 = String::from("Mohapatra");
    let u = User {
        username: &str1,
        password: &str2,
    };
    
    println!("{:?}, {:?}", u.username, u.password);
}
