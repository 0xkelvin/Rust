use std::thread;
use std::time::Duration;
//use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main () {
    let mut data = Arc::new(vec![1, 2, 3]);
    for i in 0..3 {
        // create a new owned refernece:
        let data = data.clone();

        // Use it in a thread:
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[0] += i;
        });
    }

    thread::sleep(Duration::from_millis(50));
}