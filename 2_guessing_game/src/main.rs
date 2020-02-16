// To obtain user input and then print the result as output,
// we need to bring the io (input/output) library into scope.
use std::io;

// Brings a type called std::cmp::Ordering into scope from the standard library.
use std::cmp::Ordering;

// The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.
use rand::Rng;

// The fn syntax declares a new function, the parentheses, (), 
// indicate there are no parameters, and the curly bracket, {, 
// starts the body of the function. 
fn main() {

    println!("Guess the number!");

    // The rand::thread_rng function will give us the particular random number generator that we’re going to use.
    // Then, we call the gen_range method on the random number generator. 
    // The gen_range method takes two numbers as arguments and generates a random number between them.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        // let statement is used to create a variable.
        // let mut guess will introduce a mutable variable named guess.
        // The equal sign (=) is the value that guess is bound to, 
        // which is the result of calling String::new, a function that returns a new instance of a String.
        // 
        // String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.
        // 
        // The :: syntax in the ::new line indicates that new is an associated function of the String type. 
        // Some languages call this a static method.
        // 
        // This new function creates a new, empty string.
        // 
        // let mut guess = String::new(); 
        // line has created a mutable variable that is currently bound to a new, empty instance of a String.
        let mut guess = String::new();

        // .read_line(&mut guess), calls the read_line method on the standard input handle to get input from the user.
        // We’re also passing one argument to read_line: &mut guess.
        // 
        // The & indicates that this argument is a reference,
        // which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Since Rust cannot compare a string and a number type, we must convert it.
        // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables.

        // The guess in the expression refers to the original guess that was a String with the input in it.
        // The trim method on a String instance will eliminate any whitespace at the beginning and end.

        // The parse method on strings parses a string into some kind of number.
        // Because this method can parse a variety of number types, we need to tell Rust the exact number type we want by using let guess: u32.alloc

        // The colon (:) after guess tells Rust we’ll annotate the variable’s type.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // This line prints the string we saved the user’s input in. The set of curly brackets, {}, is a placeholder.
        println!("You guessed: {}", guess);

        // The cmp method compares two values and can be called on anything that can be compared.
        // It takes a reference to whatever you want to compare with: here it’s comparing the guess to the secret_number.
        // We use a match expression to decide what to do next based on which variant of Ordering was returned from the call to 
        // cmp with the values in guess and secret_number.

        // A match expression is made up of arms. An arm consists of a pattern.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct number, the secret number was: {}", secret_number);
                break;
            },
        };
    }
}