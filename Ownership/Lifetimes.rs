struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 {self.x}
}

fn main() {
    let y = &5; // this is the same as 'let _y =5 let y=&_y
    let f = Foo { x: y};
    println!("x is: {}", f.x());
}
//As you can see, we need to declare a lifetime for Foo in the impl line.
// We repeat 'a twice, like on functions: impl<'a> defines a lifetime 'a, and Foo<'a> uses it.