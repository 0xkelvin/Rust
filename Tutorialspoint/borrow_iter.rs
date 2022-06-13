fn main() {
    let names = vec!["Kanna", "Mohtashim", "Kiran"];
    for name in names.iter() {
        match name {
            &"Mohtashim" => println!("there is a rustacean"),
            _ => println!("Hello {}", name),
        }
    }

    println!("{:?}", names);
}