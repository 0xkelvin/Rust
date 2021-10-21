fn main() {
    let v = vec![1, 2, 3, 4, 5]; // v: Vec<i32>
    let v1 = vec![0; 10]; // a vector of ten zeroes

    // accessing elements
    println!("the third element of v is {}", v[2]);

    //It’s also important to note that you must index with the usize type:
    let i:usize = 0;

    // Out-of-bounds access
    match v.get(7) {
        Some(x) => println!("Item 7 is {}", x),
        None => println!("Sorry, this vector is too short")
    }

    //Iterating
    for i in &v {
        println!("a reference to {}", i);
    }

    for i in &mut v {
        println!("a mutable reference to {}", i);
    }

    for i in v {
        println!("take ownership of the vector and its element {}", i);
    }
}