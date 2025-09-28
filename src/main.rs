fn main() {
    let ans = find_first(String::from("Ashish"));
    match ans {
        None => println!("None"),
        Some(value) => println!("{}", value),
    }
}

fn find_first(str: String) -> Option<u32> {
    let mut index = 0;
    for c in str.chars() {
        index += 1;
        if c == 'a' {
            return Some(index);
        }
    }
    None
}
