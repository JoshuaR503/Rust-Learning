// We create an instance of that struct by specifying concrete values for each of the fields.
// We create an instance by stating the name of the struct and then add curly brackets containing key: value pairs, where the keys are the names of the fields and the values are the data we want to store in those fields. 
struct User {
    username: String,
    code: u64
}

fn main() {

    let user1 = User {
        username: String::from("someusername123"),
        code: 1
    };

    // If we wanted just this user’s email address, we could use user1.email wherever we wanted to use this value. 
    println!("User: {}", user1.username);

    // If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field.
    let mut user = User {
        username: String::from("someusername123"),
        code: 2
    };

    // Mutate value.
    user.username = String::from("anotherusername");

    println!("User: {}", user.username);

    // Using struct update syntax, we can achieve the same effect with less code.
    // The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance. user1
    let user2 = User {
        username: String::from("anotherexample"),
        ..user1
    };

    print!("Code: {}\n\n", user2.code);
}

// // Listing 5-5: A build_user function that uses field init shorthand because the sername parameter has the same name as struct fields
// fn build_user(username: String) -> User {
//     User {
//         username,
//         code: 0
//     }
// }
