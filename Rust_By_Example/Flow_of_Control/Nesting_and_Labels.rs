#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Intered the outer loop");

        'inner: loop {
            println!("Intered the inner loop");

            // this would break only the inner loop
            // break;

            // this breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}