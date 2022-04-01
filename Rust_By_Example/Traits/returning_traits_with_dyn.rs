struct Sheep {}
struct Cow {}

trait Animal {
    // instance method signature
    fn noise(&self) -> &'static str;
}

// implement the 'Animal' trait for 'Sheep'
impl Anmial for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// implement the 'animal' trait for 'cow'
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "mooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}