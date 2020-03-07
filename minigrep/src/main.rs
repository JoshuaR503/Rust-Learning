// We bring the std::env module into scope with a use statement so we can use its args function.
// If your program needs to accept arguments containing invalid Unicode, use std::env::args_os instead.
use std::env;

// we need std::fs to handle files.
use std::fs;


fn main() {

    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
