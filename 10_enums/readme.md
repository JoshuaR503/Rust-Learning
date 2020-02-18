
# Rust Learning - Enums and Pattern Matching
[Enums](https://doc.rust-lang.org/book/ch06-00-enums.html#enums-and-pattern-matching) allow you to define a type by enumerating its possible variants. 

### Notes 
Enums are a feature in many languages, but their capabilities differ in each language. Rust’s enums are most similar to algebraic [data types](https://blog.softwaremill.com/algebraic-data-types-in-four-languages-858788043d4e) in functional languages, such as F#, OCaml, and Haskell.

You can put any kind of data inside an enum variant: strings, numeric types, or structs, for example. 

Note that even though the standard library contains a definition for IpAddr, we can still create and use our own definition without conflict because we haven’t brought the standard library’s definition into our scope.

The [Option](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values) type is used in many places because it encodes the very common scenario in which a value could be something or it could be nothing.

The ```Option<T>``` enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly.
The ```Option<T>``` enum is still just a regular enum, and Some(T) and None are still variants of type Option<T>. 