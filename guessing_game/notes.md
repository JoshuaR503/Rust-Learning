# Rust Learning - Notes

[Documentation](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

### Key notes
Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.

The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. ```References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references.```

The ```::``` syntax in the ```::new``` line indicates that new is an associated function of the X type0.
The set of curly brackets, ```{}```, is a placeholder.

The ```match``` construct and patterns are powerful features in Rust that let you express a variety of situations your code might encounter and make sure that you handle them all.

```Rust has a strong, static type system.```
```Rust allows us to shadow.``` This feature is often used in situations in which you want to convert a value from one type to another type. 

A few number types can have a value between 1 and 100: i32, a 32-bit number; u32, an unsigned 32-bit number; i64, a 64-bit number; as well as others. Rust defaults to an i32. 

