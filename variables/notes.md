# Rust Learning - Variables and Mutability

[Documentation](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#variables-and-mutability)

# Key notes
### Variables
When a variable is immutable, once a value is bound to a name, you can’t change that value. To illustrate this, let’s generate a new project called variables in your projects directory by using cargo new variables.

In Rust, the compiler guarantees that when you state that a value won’t change, it really won’t change. That means that when you’re reading and writing code, you don’t have to keep track of how and where a value might change. 

Variables are immutable only by default; as you did in Chapter 2, you can make them mutable by adding mut in front of the variable name. In addition to allowing this value to change, mut conveys intent to future readers of the code by indicating that other parts of the code will be changing this variable’s value.

In cases where you’re using large data structures, mutating an instance in place may be faster than copying and returning newly allocated instances. With smaller data structures, creating new instances and writing in a more functional programming style may be easier to think through, so lower performance might be a worthwhile penalty for gaining that clarity.

### Constants
You aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable.

You declare constants using the ``const`` keyword instead of the ```let`` keyword, and the type of the value must be annotated. 

Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.

Constants may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime.

```const MAX_POINTS: u32 = 100_000;```


