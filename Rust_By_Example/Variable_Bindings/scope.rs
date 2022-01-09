fn main() {
    // this binding lives in the main function
    let long_lived_binding = 1;

    // this is a block, and has a smaller scope then the main function
    {
        // this only exists in this block
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
    }
    // End of the block
    // error
    println!("outer short: {}", short_lived_binding);
    println!("outer long: {}", long_lived_binding);
}