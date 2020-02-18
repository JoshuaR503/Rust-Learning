use std::io;

const TEMP_CELSIUS: &str = "c";
const TEMP_FAHRENHEIT: &str = "f";

fn main() {
    println!("\nInsert temperature you'd like to convert");
 
    // What number do they want to conver.
    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("This is not a valid number");

    println!("What measurement would you like to convert {} to?\n", temperature);
    println!("{}: Celsius", TEMP_CELSIUS);
    println!("{}: Fahrenheit", TEMP_FAHRENHEIT);
    
    // What the user wants to do.
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("This is not a valid measurement");

    let trimmed_input = user_input.trim();
    let temperature_int: i32 = temperature
    .trim()
    .parse()
    .expect("This is not a valid number");

    println!("\nConvert {} to {}\n", temperature_int, trimmed_input);

    // Check that user input is valid.
    if trimmed_input == TEMP_CELSIUS {
        // convert to farenheit
        let result: f32 = celcuis_to_fahrenheit(temperature_int as f32);
        println!("{} {} is {} in {}", temperature_int, "Celsius", result, "Fahrenheit");

    } else if trimmed_input == TEMP_FAHRENHEIT {
        // convert to farenheit
        let result: f32 = farenheit_to_celcuis(temperature_int as f32);
        println!("{} {} is {} in {}", temperature_int, "Fahrenheit", result, "Celsius");

    } else {
        println!("Invalid input");
    }
}

fn celcuis_to_fahrenheit(c: f32) -> f32 {
    ((c * (9. / 5.)) + 32.).round()
}

fn farenheit_to_celcuis(f: f32) -> f32 {
    (((f * 5. / 9.)) - 32.).round()
}