// main.rs
use std::io;

use TemperatureConverter::{Temperature, TemperatureUnit, TEMP_OPTIONS};

fn get_conversion_type(use_from: bool) -> Option<TemperatureUnit> {
    loop {
        let mut input = String::new();
        println!(
            "What unit would you like to convert {} ({TEMP_OPTIONS})?",
            if use_from { "from" } else { "to" }
        );

        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.to_uppercase().trim() {
                "F"  | "FAHRENHEIT" => return Some(TemperatureUnit::Fahrenheit),
                "C"  | "CELSIUS"    => return Some(TemperatureUnit::Celsius),
                "K"  | "KELVIN"     => return Some(TemperatureUnit::Kelvin),
                "R"  | "RANKINE"    => return Some(TemperatureUnit::Rankine),
                "DE" | "DELISLE"    => return Some(TemperatureUnit::Delisle),
                "N"  | "NEWTON"     => return Some(TemperatureUnit::Newton),
                "RE" | "REAUMUR"    => return Some(TemperatureUnit::Reaumur),
                "QUIT" | "EXIT"     => return None,
                _ => { 
                    println!("Your input must be one of {TEMP_OPTIONS}. Please type \"quit\" to exit."); 
                    continue
                }
            },
            Err(_) => continue,  
        } 
    }
}

fn get_temperature() -> f64 {
    loop {
        let mut input = String::new();
        println!("What is the temperature you would like to convert?");
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<f64>() {
                Ok(temp) => return temp,
                Err(_) => { 
                    println!("Please enter a valid number first, even if you wish to quit."); 
                    continue;
                } 
            }, 
            Err(_) => continue,
        } 
    };
}

fn main() {
    loop {
        let from_unit: TemperatureUnit = match get_conversion_type(true) {
            Some(value) => value,
            None => break,
        };

        let to_unit: TemperatureUnit = match get_conversion_type(false) {
            Some(value) => value,
            None => break,
        };
        
        let temp_value: f64 = get_temperature();
        let temp = Temperature::new(temp_value, from_unit);
        let converted_temp = temp.convert_to(to_unit);

        println!(
            "{} = {}",
            temp.to_string(),
            converted_temp.to_string(),
        );
    }
}