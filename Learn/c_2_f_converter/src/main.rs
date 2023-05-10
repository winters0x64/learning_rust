// Celsius <=> Fahrenheit converter
use std::io;

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    let fahrenheit: f64 = (1.8 * celsius) + 32.0;

    return fahrenheit;
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    let celsius: f64 = (fahrenheit - 32.0) / 1.8;

    return celsius;
}

fn main() {
    
    loop {
        let mut choice = String::new();
        let mut temperature = String::new();
        
        println!("1) celsius");
        println!("2) fahrenheit");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice = choice.trim();

        if choice == "celsius" {
            println!("Enter the temperature: ");

            io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to read line");

            let temperature: f64 = match temperature.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let result = celsius_to_fahrenheit(temperature);

            println!("{}", result);
        } else if choice == "fahrenheit" {
            println!("Enter the temperature: ");

            io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to read line");

            let temperature: f64 = match temperature.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let result = fahrenheit_to_celsius(temperature);

            println!("{}", result);
        } else {
            println!("Enter a valid choice");
            continue;
        }
    }
}
