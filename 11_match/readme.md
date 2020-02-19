# Rust Learning - The match Control Flow Operator

[Documentation](https://doc.rust-lang.org/book/ch06-02-match.html)

### Notes
Rust has an extremely powerful control flow operator called ```match``` that allows you to compare a value against a series of patterns and then execute code based on which pattern matches. The power of ```match``` comes from the expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled.

Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid. Especially in the case of Option<T>, when Rust prevents us from forgetting to explicitly handle the None case, it protects us from assuming that we have a value when we might have null, thus making the billion-dollar mistake discussed earlier.

[The _ Placeholder](https://doc.rust-lang.org/book/ch06-02-match.html#the-_-placeholder)
Rust also has a pattern we can use when we don’t want to list all possible values. 