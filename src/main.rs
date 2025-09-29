fn main() {
    let s1 = sum(1.1, 2.2);
    let s2 = sum(1, 2);
    let s3 = sum(true, false);    //cannot add bool to bool, because bool does not implement the Add trait
    println!("{}", s2);
    println!("{}", s1);
}

fn sum<T: std::ops::Add<Output =T>>(a:T, b:T) -> T{
    return a + b;
}
