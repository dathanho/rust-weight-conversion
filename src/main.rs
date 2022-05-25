use std::io;

// Start of function
fn main() {
    println!("Weight conversion tool (Milligrams, Grams, Kilograms, Ounces, Pounds):");

    // Getting input for unit (from)
    println!("What unit do you want to convert from?");

    let mut unit_from = String::new();

    io::stdin()
        .read_line(&mut unit_from)
        .expect("Failed to read line");

    // Getting input for unit (to)
    println!("What unit do you want to convert to");

    let mut unit_to = String::new();

    io::stdin()
        .read_line(&mut unit_to)
        .expect("Failed to read line");

    // Getting amount
    println!(
        "Enter the amount of {} you want to convert.",
        unit_from.trim().to_lowercase()
    );

    let mut input_amount = String::new();

    io::stdin()
        .read_line(&mut input_amount)
        .expect("Failed to read line");

    let input_amount: f64 = input_amount.trim().parse().unwrap();

    // Units conversion
    let milligrams = 0.001;
    let grams = 1.0;
    let kilograms = 1000.0;
    let ounces = 28.3495;
    let pounds = 453.592;

    // Converting from milligrams
    if unit_from.trim().to_lowercase() == "milligrams" {
        // THE MATH
        if unit_to.trim().to_lowercase() == "milligrams" {
            let x = input_amount * 1.0;
            println!("{} milligram(s) is {} milligram(s).", input_amount, x)
        }
        if unit_to.trim().to_lowercase() == "grams" {
            let x = input_amount * milligrams / grams;
            println!("{} milligram(s) is {} gram(s).", input_amount, x)
        }
        if unit_to.trim().to_lowercase() == "kilograms" {
            let x = input_amount * milligrams / kilograms;
            println!("{} milligram(s) is {} kilogram(s).", input_amount, x)
        }
        if unit_to.trim().to_lowercase() == "ounces" {
            let x = input_amount * milligrams / ounces;
            println!("{} milligram(s) is {:.5} ounce(s).", input_amount, x)
        }
        if unit_to.trim().to_lowercase() == "pounds" {
            let x = input_amount * milligrams / pounds;
            println!("{} milligram(s) is {:.5} pound(s).", input_amount, x)
        }
    }

    // Converting from grams
    if unit_from.trim().to_lowercase() == "grams" {
        // THE MATH
        if unit_to.trim().to_lowercase() == "milligrams" {
            let x = input_amount / milligrams;
            println!("{} gram(s) is {} milligram(s).", input_amount, x);
        }
        if unit_to.trim().to_lowercase() == "grams" {
            let x = input_amount * 1.0;
            println!("{} gram(s) is {} gram(s)", input_amount, x)
        }
        if unit_to.trim().to_lowercase() == "kilograms" {
            let x = input_amount / kilograms;
            println!("{} gram(s) is {} kilogram(s).", input_amount, x);
        }
        if unit_to.trim().to_lowercase() == "ounces" {
            let x = input_amount / ounces;
            println!("{} gram(s) is {:.5} ounce(s).", input_amount, x)
        }
        if unit_to.trim().to_lowercase() == "pounds" {
            let x = input_amount / pounds;
            println!("{} gram(s) is {:.5} pound(s).", input_amount, x)
        }
    }

    // Converting from kilograms
    if unit_from.trim().to_lowercase() == "kilograms" {
        // THE MATH
        if unit_to.trim().to_lowercase() == "miligrams" {
            let x = input_amount * kilograms / milligrams;
            println!("{} kilogram(s) is {} milligram(s).", input_amount, x)
        }
        if unit_to.trim().to_lowercase() == "grams" {
            let x = input_amount * kilograms / grams;
            println!("{} kilogram(s) is {} gram(s).", input_amount, x)
        }
        if unit_to.trim().to_lowercase() == "kilograms" {
            let x = input_amount * 1.0;
            println!("{} kilogram(s) is {} kilogram(s).", input_amount, x)
        }
        if unit_to.trim().to_lowercase() == "ounces" {
            let x = input_amount * kilograms / ounces;
            println!("{} kilogram(s) is {} ounce(s).", input_amount, x)
        }
        if unit_to.trim().to_lowercase() == "pounds" {
            let x = input_amount * kilograms / pounds;
            println!("{} kilogram(s) is {} pound(s).", input_amount, x)
        }
    }

    // Converting from ounces
    if unit_from.trim().to_lowercase() == "ounces" {
        // THE MATH
        if unit_to.trim().to_lowercase() == "milligrams" {
            let x = input_amount * ounces / milligrams;
            println!("{} ounce(s) is {} milligram(s).", input_amount, x)
        }
        if unit_to.trim().to_lowercase() == "grams" {
            let x = input_amount * ounces / grams;
            println!("{} ounce(s) is {} gram(s).", input_amount, x)
        }
        if unit_to.trim().to_lowercase() == "kilograms" {
            let x = input_amount * ounces / kilograms;
            println!("{} ounce(s) is {:.5} kilogram(s).", input_amount, x)
        }
        if unit_to.trim().to_lowercase() == "ounces" {
            let x = input_amount * 1.0;
            println!("{} ounce(s) is {} ounce(s)", input_amount, x)
        }
        if unit_to.trim().to_lowercase() == "pounds" {
            let x = input_amount * ounces / pounds;
            println!("{} ounce(s) is {} pound(s).", input_amount, x)
        }
    }

    // Converting from pounds
    if unit_from.trim().to_lowercase() == "pounds" {
        // THE MATH
        if unit_to.trim().to_lowercase() == "milligrams" {
            let x = input_amount * pounds / milligrams;
            println!("{} pound(s) is {} milligram(s).", input_amount, x)
        }
        if unit_to.trim().to_lowercase() == "grams" {
            let x = input_amount * pounds / grams;
            println!("{} pound(s) is {} gram(s).", input_amount, x)
        }
        if unit_to.trim().to_lowercase() == "kilograms" {
            let x = input_amount * pounds / kilograms;
            println!("{} pound(s) is {:.5} kilogram(s).", input_amount, x)
        }
        if unit_to.trim().to_lowercase() == "ounces" {
            let x = input_amount * pounds / ounces;
            println!("{} pound(s) is {} ounce(s).", input_amount, x)
        }
        if unit_to.trim().to_lowercase() == "pounds" {
            let x = input_amount * 1.0;
            println!("{} pound(s) is {} pound(s).", input_amount, x)
        }
    }
}