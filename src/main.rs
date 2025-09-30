use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    age: u32,
}
fn main() {
    let u = User {
        username: String::from("Ashish"),
        age: 23,
    };

    let serialized_string = serde_json::to_string(&u);


    match serialized_string{
        Ok(str)=>{
            println!("{}", str);
        },
        Err(e)=>{
            println!("Error: {}", e);
        }
    }
}
