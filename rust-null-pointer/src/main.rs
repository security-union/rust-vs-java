fn main() {
    let s: Option<String> = None;

    match s {
        Some(i) => println!("Some: {}", i),
        None => println!("None"),
    }
}