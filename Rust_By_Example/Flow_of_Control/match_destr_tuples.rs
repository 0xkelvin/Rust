fn main() {
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);
    // Match can be used to destructure a tuple
    match triple {
        // destructure the second and third elements
        (0, y, z) => println!("First is '0', 'y' is {:?}, and 'z' is {:?}", y, z),
        (1, ..) => println!("Firs is '1' and the rest doesn't matter"),
        // '..' can be used to ignore the rest of the tuple
        _ => println!("It doesn't matter what they are"),
        // '_' means dont bind the value to a variable
    }
}