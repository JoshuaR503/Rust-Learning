


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

    
}
