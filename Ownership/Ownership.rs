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