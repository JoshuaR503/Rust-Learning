fn main() {
    another_function(5, 6);

    let x = five();
    let total = plus_one(902);

    println!("Total: {}", total);
    println!("The value of X is: {}", x);
}

// The declaration of another_function has one parameter named x. The type of x is specified as i32.
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// Functions can return values to the code that calls them. We don’t name return values, but we do declare their type after an arrow (->). 
fn five() -> i32 {
    55
}

// Experssions 
fn plus_one(x: i32) -> i32 {
    x * 1
}