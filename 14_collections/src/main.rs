
use std::collections::HashMap;

fn main() {


    // Vectors are implemented using generics.
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];

    // To create a vector and then add elements to it, we can use the push method.
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Creating a String.
    // This line creates a new empty string called s, which we can then load data into.
    let s1 = String::new();

    // the method also works on a literal directly:
    let s2 = "initial contents".to_string();

    // In this case, String::from and to_string do the same thing, so which you choose is a matter of style.
    let s3 = String::from("initial contents");



    // Creating a hash map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = 
    teams
    .iter()
    .zip(initial_scores.iter())   // create a vector of tuples where “Blue” is paired with 10, and so forth. 
    .collect();                   // use the collect method to turn that vector of tuples into a hash map.
}
