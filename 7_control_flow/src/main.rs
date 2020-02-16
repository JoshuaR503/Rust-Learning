fn main() {

    if_else_elseif();

    loops();

    while_loop();

    for_loop();
}

fn if_else_elseif () {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let x = 6;

    // Multiple conditions.
    if x % 4 == 0 {
        println!("x is divisible by 4");
    } else if x % 3 == 0 {
        println!("x is divisible by 3");
    } else if x % 2 == 0 {
        println!("x is divisible by 2");
    } else {
        println!("x is not divisible by 4, 3, or 2");
    }

    // If in a let statement
    let condition = true;
    let y = if condition {
        5
    } else {
        6
    };

    println!("y is: {}", y);
}

fn loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop () {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}