use std::sync::{Arc,Mutex};
use std::{thread, time};
fn main() {
    let pointer = Arc::new(Mutex::new(12));
    let second_pointer = pointer.clone(); // or Arc::clone(&pointer)
    thread::spawn(move || {
        let mut mutable_pointer = second_pointer.lock().unwrap();
        *mutable_pointer = 1;
    });
    thread::sleep(time::Duration::from_secs(10));
    let one = pointer.lock().unwrap();
    println!("{}", one);
}