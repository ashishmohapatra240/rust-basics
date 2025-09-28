fn main() {
    let mut s1: String = String::from("Ashish");
    let s2 = &mut s1;
    let s3 = &s1;
    let s4 = &s1;

    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
    println!("{}", s4);



}
