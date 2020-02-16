# Rust Learning - Data Types

[Documentation](https://doc.rust-lang.org/book/ch03-02-data-types.html#data-types)

Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data. We’ll look at two data type subsets: scalar and compound.

Keep in mind that Rust is a statically typed language, which means that it must know the types of all variables at compile time.

## Scalar Types
A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. 

## Integer Types
An integer is a number without a fractional component. This type declaration indicates that the value it’s associated with should be an unsigned integer (signed integer types start with i, instead of u) that takes up 32 bits of space.

Each variant can be either signed or unsigned and has an explicit size.

```Signed``` and ```unsigned``` refer to whether it’s possible for the number to be negative or positive—in other words, whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a sign (unsigned).

``` unsigned = positive ```
``` signed = negative ```

## So how do you know which type of integer to use?
If you’re unsure, Rust’s defaults are generally good choices, and integer types default to i32: this type is generally the fastest, even on 64-bit systems. The primary situation in which you’d use isize or usize is when indexing some sort of collection.

## Floating-Point Types
Rust also has two primitive types for floating-point numbers, which are numbers with decimal points.
Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively.

## Numeric Operations
Rust supports the basic mathematical operations you’d expect for all of the number types: addition, subtraction, multiplication, division, and remainder.

## The Boolean Type
The main way to use Boolean values is through conditionals, such as an if expression.

## The Character Type
Rust’s char type is the language’s most primitive alphabetic type, and the following code shows one way to use it.
Char literals are specified with single quotes, as opposed to string literals, which use double quotes.

Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust.

## Compound Types
Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

### The Tuple Type
A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

We create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.

### Array type
Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length, like tuples.

An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size. 