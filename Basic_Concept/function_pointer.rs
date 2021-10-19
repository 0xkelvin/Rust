fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32{
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

}
/*An example of a case where you’d want to only accept fn and not closures is when interfacing with external code that doesn’t have closures: 
C functions can accept functions as arguments, but C doesn’t have closures.

For an example where we can use either a closure defined inline or a named function, 
let’s look at a use of map. To use the map function to turn a vector of numbers into a vector of strings, we could use a closure: */
let list_of_numbers = vec![1,2,3];
let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(|i| i.to_string())
    .collect();

/*Or we could name a function as the argument to map instead of the closure: */
let list_of_numbers = vec![1,2,3];
let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(ToString::to_string())
    .collect();