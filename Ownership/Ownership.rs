/*
All programs have to manage the way they use a computer’s memory while running. 
Some languages have garbage collection that constantly looks for no longer used memory as the program runs; 
in other languages, the programmer must explicitly allocate and free the memory. 
Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks at compile time. 
None of the ownership features slow down your program while it’s running.

Because ownership is a new concept for many programmers, it does take some time to get used to. 
The good news is that the more experienced you become with Rust and the rules of the ownership system, the more you’ll be able to naturally develop code that is safe and efficient.
Keep at it!

Stack : all data stored on the stack must have a known, fixed size
Heap  : data with unknown size at compile time or a size that might change must be stored on the heap instead


https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

***Ownership Rules

First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:
- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

 */

 fn main()
 {
    // s is not valid here, it’s not yet declared
     let s = "hello"; // s is valid from this point forward
     let mut s1 = String::from("tring::from hello"); //This type is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time
    // do stuff with s
     s1.push_str(", world !");  // push_str() appends a literal to a String
     printlt!("{}", s); // this will print out
 }// this scope is now over, and s is no longer valid

 /*
 MEMORY and ALLOCATION
 In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. 
 This is why string literals are fast and efficient. 
 But these properties only come from the string literal’s immutability. 
 Unfortunately, we can’t put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program.

With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. 
This means:
 - The memory must be requested from the memory allocator at runtime.
 - We need a way of returning this memory to the allocator when we’re done with our String.
That first part is done by us: when we call String::from, its implementation requests the memory it needs. This is pretty much universal in programming languages.

However, the second part is different. 
In languages with a garbage collector (GC), the GC keeps track and cleans up memory that isn’t being used anymore, and we don’t need to think about it. 
Without a GC, it’s our responsibility to identify when memory is no longer being used and call code to explicitly return it, just as we did to request it.
 Doing this correctly has historically been a difficult programming problem. 
 If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable. 
 If we do it twice, that’s a bug too. We need to pair exactly one allocate with exactly one free.
 
 
 */