fn main() {
    println!("Hello, world!");

    let a = String::from("123");
    // let b = a.clone();
    let b = a.clone();
    let c = a.clone();
    // let c = a;
    println!("{}", b);
    println!("{}", c);
}
