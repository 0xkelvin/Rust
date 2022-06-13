fn main() {
    let mut names = vec!["Kannan", "Mohtashim", "Kiran"];
    for name in names.iter_mut() {
       match name {
          &mut "Mohtashim" => println!("There is a rustacean among us!"),
          _ => println!("Hello {}", name),
       }
    }
    println!("{:?}",names);
    //// reusing the collection after iteration
 }