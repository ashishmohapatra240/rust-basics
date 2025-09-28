fn main() {
    let ans = sum(1, 2);
    println!("{}", ans);
    let is_even = is_even(1);
    println!("{}", is_even);

    let name = String::from("Ashish");
    println!("{}", name);

    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", v);

    for i in 0..100 {
        print!("{} ", i);
    }

    let mut a = 1;
    a = 2;
    println!("{}", a);

    let mut name: String = String::from("Ashish");

    let name2 = name.clone();          //dangling pointer error
    name.push_str(" Mohapatra");
    println!("{}", name2);

    let name3 = String::from("Ashish");
    let (len, name3) = get_len(name3);
    println!("{}", name3);
    println!("{}", len);
}

fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}

fn is_even(a: u32) -> bool {
    return a % 2 == 0;
}

fn get_len(s: String) -> (usize, String) {
    return (s.len(), s);
}
