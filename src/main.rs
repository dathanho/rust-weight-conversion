use std::io;

// Start of function
fn main() {
    println!("Weight conversion tool (Milligrams, Grams, Kilograms, Ounces, Pounds):");

    // Getting input for unit (from)
    println!("What unit do you want to convert from?");

    let unit_from: String = loop {
        let mut unit = String::new();

        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line");

        if unit.trim().to_lowercase() == "milligrams" {
            break unit;
        }
        if unit.trim().to_lowercase() == "grams" {
            break unit;
        }
        if unit.trim().to_lowercase() == "kilograms" {
            break unit;
        }
        if unit.trim().to_lowercase() == "ounces" {
            break unit;
        }
        if unit.trim().to_lowercase() == "pounds" {
            break unit;
        } else {
            println!("Please input a supported unit.");
            continue;
        }
    };
    // Getting input for unit (to)
    println!("What unit do you want to convert to?");

    let unit_to: String = loop {
        let mut unit = String::new();

        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line");

        if unit.trim().to_lowercase() == "milligrams" {
            break unit;
        }
        if unit.trim().to_lowercase() == "grams" {
            break unit;
        }
        if unit.trim().to_lowercase() == "kilograms" {
            break unit;
        }
        if unit.trim().to_lowercase() == "ounces" {
            break unit;
        }
        if unit.trim().to_lowercase() == "pounds" {
            break unit;
        } else {
            println!("Please input a supported unit.");
            continue;
        };
    };
    // Getting amount
    println!(
        "Enter the amount of {} you want to convert.",
        unit_from.trim().to_lowercase()
    );

    let input_amount: f64 = loop {
        println!("Input a number.");
        let mut amount = String::new();

        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read line");

        let amount: f64 = match amount.trim().parse() {
            Ok(amount) => amount,
            Err(_) => continue,
        };
        break amount;
    };

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
            let answer = input_amount * 1.0;
            println!("{} milligram(s) is {} milligram(s).", input_amount, answer)
        }
        if unit_to.trim().to_lowercase() == "grams" {
            let answer = input_amount * milligrams / grams;
            println!("{} milligram(s) is {} gram(s).", input_amount, answer)
        }
        if unit_to.trim().to_lowercase() == "kilograms" {
            let answer = input_amount * milligrams / kilograms;
            println!("{} milligram(s) is {} kilogram(s).", input_amount, answer)
        }
        if unit_to.trim().to_lowercase() == "ounces" {
            let answer = input_amount * milligrams / ounces;
            println!("{} milligram(s) is {:.5} ounce(s).", input_amount, answer)
        }
        if unit_to.trim().to_lowercase() == "pounds" {
            let answer = input_amount * milligrams / pounds;
            println!("{} milligram(s) is {:.5} pound(s).", input_amount, answer)
        }
    }

    // Converting from grams
    if unit_from.trim().to_lowercase() == "grams" {
        // THE MATH
        if unit_to.trim().to_lowercase() == "milligrams" {
            let answer = input_amount / milligrams;
            println!("{} gram(s) is {} milligram(s).", input_amount, answer);
        }
        if unit_to.trim().to_lowercase() == "grams" {
            let answer = input_amount * 1.0;
            println!("{} gram(s) is {} gram(s)", input_amount, answer)
        }
        if unit_to.trim().to_lowercase() == "kilograms" {
            let answer = input_amount / kilograms;
            println!("{} gram(s) is {} kilogram(s).", input_amount, answer);
        }
        if unit_to.trim().to_lowercase() == "ounces" {
            let answer = input_amount / ounces;
            println!("{} gram(s) is {:.5} ounce(s).", input_amount, answer)
        }
        if unit_to.trim().to_lowercase() == "pounds" {
            let answer = input_amount / pounds;
            println!("{} gram(s) is {:.5} pound(s).", input_amount, answer)
        }
    }

    // Converting from kilograms
    if unit_from.trim().to_lowercase() == "kilograms" {
        // THE MATH
        if unit_to.trim().to_lowercase() == "miligrams" {
            let answer = input_amount * kilograms / milligrams;
            println!("{} kilogram(s) is {} milligram(s).", input_amount, answer)
        }
        if unit_to.trim().to_lowercase() == "grams" {
            let answer = input_amount * kilograms / grams;
            println!("{} kilogram(s) is {} gram(s).", input_amount, answer)
        }
        if unit_to.trim().to_lowercase() == "kilograms" {
            let answer = input_amount * 1.0;
            println!("{} kilogram(s) is {} kilogram(s).", input_amount, answer)
        }
        if unit_to.trim().to_lowercase() == "ounces" {
            let answer = input_amount * kilograms / ounces;
            println!("{} kilogram(s) is {} ounce(s).", input_amount, answer)
        }
        if unit_to.trim().to_lowercase() == "pounds" {
            let answer = input_amount * kilograms / pounds;
            println!("{} kilogram(s) is {} pound(s).", input_amount, answer)
        }
    }

    // Converting from ounces
    if unit_from.trim().to_lowercase() == "ounces" {
        // THE MATH
        if unit_to.trim().to_lowercase() == "milligrams" {
            let answer = input_amount * ounces / milligrams;
            println!("{} ounce(s) is {} milligram(s).", input_amount, answer)
        }
        if unit_to.trim().to_lowercase() == "grams" {
            let answer = input_amount * ounces / grams;
            println!("{} ounce(s) is {} gram(s).", input_amount, answer)
        }
        if unit_to.trim().to_lowercase() == "kilograms" {
            let answer = input_amount * ounces / kilograms;
            println!("{} ounce(s) is {:.5} kilogram(s).", input_amount, answer)
        }
        if unit_to.trim().to_lowercase() == "ounces" {
            let answer = input_amount * 1.0;
            println!("{} ounce(s) is {} ounce(s)", input_amount, answer)
        }
        if unit_to.trim().to_lowercase() == "pounds" {
            let answer = input_amount * ounces / pounds;
            println!("{} ounce(s) is {} pound(s).", input_amount, answer)
        }
    }

    // Converting from pounds
    if unit_from.trim().to_lowercase() == "pounds" {
        // THE MATH
        if unit_to.trim().to_lowercase() == "milligrams" {
            let answer = input_amount * pounds / milligrams;
            println!("{} pound(s) is {} milligram(s).", input_amount, answer)
        }
        if unit_to.trim().to_lowercase() == "grams" {
            let answer = input_amount * pounds / grams;
            println!("{} pound(s) is {} gram(s).", input_amount, answer)
        }
        if unit_to.trim().to_lowercase() == "kilograms" {
            let answer = input_amount * pounds / kilograms;
            println!("{} pound(s) is {:.5} kilogram(s).", input_amount, answer)
        }
        if unit_to.trim().to_lowercase() == "ounces" {
            let answer = input_amount * pounds / ounces;
            println!("{} pound(s) is {} ounce(s).", input_amount, answer)
        }
        if unit_to.trim().to_lowercase() == "pounds" {
            let answer = input_amount * 1.0;
            println!("{} pound(s) is {} pound(s).", input_amount, answer)
        }
    }
}
