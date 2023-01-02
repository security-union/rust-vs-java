use std::cell::RefCell;

fn main() {
    let x = RefCell::new("sdfsdf");
    let y = x.borrow_mut();
    let w = x.borrow_mut();
    println!("{}", y);
}