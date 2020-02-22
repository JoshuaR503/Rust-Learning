# Rust Learning - Common Collections

[Documentation](https://doc.rust-lang.org/book/ch08-00-common-collections.html)

### Notes
A vector allows you to store a variable number of values next to each other.
A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
A hash map allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.

### Vectors
[Storing Lists of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html#storing-lists-of-values-with-vectors)
Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. 
Vectors can only store values of the same type. They are useful when you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.
```let v: Vec<i32> = Vec::new();```

There are two ways to [reference a value stored in a vector](https://doc.rust-lang.org/book/ch08-01-vectors.html#reading-elements-of-vectors).
First, we use the index value of 2 to get the third element: vectors are indexed by number, starting at zero. Second, the two ways to get the third element are by using & and [], which gives us a reference, or by using the get method with the index passed as an argument, which gives us an Option<&T>