// a type borrowed which houses a reference to an i32, the reference 
// to i32 must outlive borrowed
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// similarly, both references here must outlive this structure
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32, 
}

// An number which is either an i32 or a reference to one
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed{x: &x, y: &y};
    let reference = Either::Ref(&x);
    let number    = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);

}