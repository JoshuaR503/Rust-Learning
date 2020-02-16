# Rust Learning - Control Flow

[Documentation](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

### if, else, if else
Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean.

Using too many else if expressions can clutter your code, so if you have more than one, you might want to refactor your code. Chapter 6 describes a powerful Rust branching construct called ```match``` for these cases.

### loop
Rust provides several loops.

Rust has three kinds of loops: loop, while, and for. Let’s try each one.
The ```loop``` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.

### while
This construct eliminates a lot of nesting that would be necessary if you used loop, if, else, and break, and it’s clearer. While a condition holds true, the code runs; otherwise, it exits the loop.

### for
The safety and conciseness of for loops make them the most commonly used loop construct in Rust. 