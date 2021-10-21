use std::thread;

fn main() {
    let x = 1;
    let handle = thread::spawn( move || {
        println!("x is {}", x);
    });

    println!("{}", handle.join().unwrap());
}