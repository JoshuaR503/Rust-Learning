# Rust Learning

### Notes
By default variables are immutable. This is one of many nudges Rust gives you to write your code in a way that takes advantage of the safety and easy concurrency that Rust offers. 

Arrays in Rust have a fixed length, like tuples.

Rust is an expression-based language, this is an important distinction to understand.

Ownership is Rust’s most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector. Rust’s central feature is ownership. Although the feature is straightforward to explain, it has deep implications for the rest of the language. [Read about Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html#understanding-ownership)

Putting the specifier ```:?``` inside the curly brackets tells println! we want to use an output format called Debug.

Rust has a feature called [automatic referencing and dereferencing](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#wheres-the---operator). Calling methods is one of the few places in Rust that has this behavior.

[Read about Integer Overflow](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow)

[Read about Tuple Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type)

[Read about Function Experssions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#function-bodies-contain-statements-and-expressions)

### Rust conventions
Rust’s naming convention for constants is to use all uppercase with underscores between words, and underscores can be inserted in numeric literals to improve readability.

Rust code uses snake case as the conventional style for function and variable names.

