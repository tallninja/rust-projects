use std::io;

fn main() {
    let mut temp: String = String::new();

    let mut option = String::new();

    println!("1. Degrees to Fahrenheit");
    println!("2. Fahrenheit to Degrees");

    println!("Select an option:");

    io::stdin().read_line(&mut option).expect("Enter a digit !");

    let option: u32 = option.trim().parse().expect("Enter a value");

    println!("Enter the temperature:");
    io::stdin().read_line(&mut temp).expect("Enter a value !");
    let temp: f32 = temp.trim().parse().expect("Enter a temperature");
    println!("Teperature entered: {temp}");

    if option == 1 {
        println!("Converted: {}", degrees_to_fahrenheit(&temp));
    } else if option == 2 {
        println!("Converted: {}", fahrenheit_to_degrees(&temp));
    } else {
        println!("Enter a valid option !");
    }
}

fn fahrenheit_to_degrees(temp: &f32) -> f32 {
    (temp - 32.0) / 1.8
}

fn degrees_to_fahrenheit(temp: &f32) -> f32 {
    (temp * 1.8) + 32.0
}
