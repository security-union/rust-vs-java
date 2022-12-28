fn main() {
    println!("Hello, world!");
    let dangling_pointer = vec![1, 2, 3];
    drop(dangling_pointer);
    println!("{:?}", dangling_pointer);
}
