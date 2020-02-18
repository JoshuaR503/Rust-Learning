# Rust Learning - Using Tuple Structs without Named Fields to Create Different Types
[Documentation](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types)

Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields.
Tuple structs are useful when you want to give the whole tuple a name and make the tuple be a different type from other tuples, and naming each field as in a regular struct would be verbose or redundant.

### Unit-Like Structs Without Any Fields
[Documentation](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#unit-like-structs-without-any-fields)

You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to (), the unit type.

### Ownership of Struct Data
[Documentation](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#ownership-of-struct-data)
It’s possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes. 
Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.
