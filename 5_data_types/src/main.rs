fn main() {

    // Floating-Point Types
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    println!("x: {}", x);
    println!("y: {}", y);

    // Numeric Operations

    // addition
    let sum = 5 + 10;
    println!("Sum: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("Substraction: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("Sultiplication: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("Division: {}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("Remainder: {}", remainder);

    // Booleans
    let t = true;
    println!("true: {}", t);

    let f: bool = false; // with explicit type annotation
    println!("false: {}", f);

    // Chars
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}, {}, {}", c, z, heart_eyed_cat);

    // Tuple
    // The variable tup binds to the entire tuple, because a tuple is considered a single compound element. 
    // To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of y is: {}, \nx is: {}, \nz is {}", y, x, z);

    // Array
    let a = [1, 2, 3, 4, 5];
    println!("Array: {}", a[1]);

    // An example of when you might want to use an array rather than a vector is in a program that needs to know the names of the months of the year. It’s very unlikely that such a program will need to add or remove months, so you can use an array because you know it will always contain 12 elements:
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
}
