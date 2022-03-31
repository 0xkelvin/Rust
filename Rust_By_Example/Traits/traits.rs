struct Sheep {
    naked: bool,
    name: &'static str
}

trait Animal {
    // associated function signature; 'Self' refers to the implementor type.__rust_force_exp type.
    fn new(name: &'static str) -> Self;

    // Method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked;
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // implementor methods can use the implementor's trait methods
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

// implement the 'Animal' trait for 'Sheep'
impl Animal for Sheep {
    // 'Self' is the implementor type: 'Sheep'
    fn new(name: &'static str) -> Sheep {
        Sheep {name: name, naked: false}
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked(){
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }

    // default trait methods can be overridden
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    // type annotation is necessary in this case
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}