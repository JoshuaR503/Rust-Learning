# Rust Learning - An Example Program Using Structs

A program that calculates the area of a rectangle. [Documentation](https://doc.rust-lang.org/book/ch05-02-example-structs.html)

### Notes
Putting the specifier ```:?``` inside the curly brackets tells println! we want to use an output format called Debug.

The Debug trait enables us to print our struct in a way that is useful for developers so we can see its value while we’re debugging our code.

Rust has provided a number of traits for us to use with the derive annotation that can add useful behavior to our custom types. 

Each struct is allowed to have multiple impl blocks.

## Method Syntax
Methods are similar to functions. [Documentation](https://doc.rust-lang.org/book/ch05-03-method-syntax.html) Methods are different from functions in that they’re defined within the context of a struct (or an ```enum``` or a trait ```object```, which we cover in Chapters 6 and 17, respectively), and their first parameter is always ```self```, which represents the instance of the struct the method is being called on.

### Defining Methods 
[Documentation](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#defining-methods)

### Associated Functions
[Documentation](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#associated-functions)
Another useful feature of impl blocks is that we’re allowed to define functions within impl blocks that don’t take self as a parameter.

These are called associated functions because they’re associated with the struct. 

They’re still functions, not methods, because they don’t have an instance of the struct to work with. 

Associated functions are often used for constructors that will return a new instance of the struct.


