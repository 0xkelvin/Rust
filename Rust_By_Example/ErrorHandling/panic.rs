fn drink(beverage: &str) {
    if beverage == "lemonade" { panic!("AAAaaa!!!");}

    println!("Some refreshing {} is all I need.", beverage);
}

fn main() {
    drink("water");
    drink("lemonade");
}