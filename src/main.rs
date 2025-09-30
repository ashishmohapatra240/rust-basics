use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
struct User {
    username: String,
    age: u32,
}

fn main() {
    let u = User {
        username: String::from("Ashish"),
        age: 23,
    };
    let mut v: Vec<u8> = Vec::new();
    let ans = u.serialize(&mut v);

    match ans {
        Ok(()) => {
            println!("{:?}", ans);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
