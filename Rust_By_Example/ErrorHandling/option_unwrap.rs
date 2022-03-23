fn give_adult(drink: Option<&str>) {
    // specify a course of action for each case
    match drink {
        Some("lemonable") => println!("Yuck! Too sugary."),
        Some(inner) => println!("{}? how nice", inner);
        None        => println!("No drink? Oh well"),
    }
}

fn drink(drink: Option<&str>) {
    // unwrap returns a panic when it receives a None
    let inside = drink.unwrap();
    if inside == "lemonade" {panic!("Aa");}

    println!("I love {}s!",inside);
}

fn main() {
    let water = Some("water");
    let lemonable = Some("lemonade");
    let void = None;

    give_adult(water);
    give_adult(lemonable);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    drink(nothing);
}