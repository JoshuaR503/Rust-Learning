fn main() {

    // The syntax if let takes a pattern and an expression separated by an equal sign. 
    // It works the same way as a match, where the expression is given to the match and the pattern is its first arm.

    if let Some(3) = some_u8_value {
        println!("three");
    }

}
