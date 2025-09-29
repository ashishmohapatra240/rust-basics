use std::fmt::Display;

struct User {
    username: String,
    age: u32,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.username)
    }
}

fn main() {
    let u = User {
        username: String::from("Ashish"),
        age: 23,
    };

    print!("{}", u.username);
}
