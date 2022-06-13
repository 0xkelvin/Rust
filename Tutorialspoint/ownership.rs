fn main() {
    let names = vec!["Kannan", "Mohtashim", "Kiran"];
    for name in names.into_iter(){
        match name {
            "Mohtashim" => println!("This is a rustance"),
            _ => println!("Hello {}", name),
        }
    }
    //println!("{:?}",names); 
   //Error:Cannot access after ownership move
}