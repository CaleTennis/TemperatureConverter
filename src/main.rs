use std::io;

const DEGREE_SYMBOL: &str = "\u{00B0}";

fn format_with_commas(number: f64) -> String {
    let number_string = format!("{:.2}", number.abs());
    let float_parts: Vec<&str> = number_string.split('.').collect();
    let integer_part = float_parts[0];
    let decimal_part = float_parts.get(1).unwrap();

    let len = integer_part.len();
    let mut result = String::new();

    for (i, c) in integer_part.char_indices() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }

    result.push('.');
    result.push_str(decimal_part);

    if number.is_sign_negative() {
        format!("-{}", result)
    } else {
        result
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum TemperatureUnit {
    Fahrenheit,
    Celsius,
    Kelvin,
    Rankine
}

impl TemperatureUnit {
    fn to_string(self) -> String {
        match self {
            TemperatureUnit::Fahrenheit => format!("{DEGREE_SYMBOL}F"),
            TemperatureUnit::Celsius => format!("{DEGREE_SYMBOL}C"),
            TemperatureUnit::Kelvin => format!("K"),
            TemperatureUnit::Rankine => format!("{DEGREE_SYMBOL}R"),
        }
    }
}

struct Temperature {
    value: f64,
    unit: TemperatureUnit,
}

impl Temperature {
    fn new(value: f64, unit: TemperatureUnit) -> Self {
        Temperature { value, unit }
    }

    fn convert_to(&self, target_unit: TemperatureUnit) -> Temperature {
        let value_in_target_unit = match (self.unit, target_unit) {
            (TemperatureUnit::Fahrenheit, TemperatureUnit::Celsius) => (self.value - 32.0) / 1.8,
            (TemperatureUnit::Fahrenheit, TemperatureUnit::Kelvin) => ((self.value - 32.0) / 1.8) + 273.15,
            (TemperatureUnit::Fahrenheit, TemperatureUnit::Rankine) => self.value + 459.67,
    
            (TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit) => (self.value * 1.8) + 32.0,
            (TemperatureUnit::Celsius, TemperatureUnit::Kelvin) => self.value + 273.15,
            (TemperatureUnit::Celsius, TemperatureUnit::Rankine) => (self.value * 1.8) + 491.67,
    
            (TemperatureUnit::Kelvin, TemperatureUnit::Fahrenheit) => ((self.value - 273.15) * 1.8) + 32.0,
            (TemperatureUnit::Kelvin, TemperatureUnit::Celsius) => self.value - 273.15,
            (TemperatureUnit::Kelvin, TemperatureUnit::Rankine) =>  ((self.value - 273.15) * 1.8) + 491.67,
    
            (TemperatureUnit::Rankine, TemperatureUnit::Fahrenheit) => self.value - 459.67,
            (TemperatureUnit::Rankine, TemperatureUnit::Celsius) => (self.value - 491.67) / 1.8,
            (TemperatureUnit::Rankine, TemperatureUnit::Kelvin) =>  ((self.value - 491.67) / 1.8) + 273.15,

            // To & from the same unit
            (_, _) => self.value,
        };

        Temperature::new(value_in_target_unit, target_unit)
    }

    fn to_string(&self) -> String {
        format!("{}{}", format_with_commas(self.value), self.unit.to_string())
    }
}

fn get_conversion_type(use_from: bool) -> Option<TemperatureUnit> {
    loop {
        let mut input = String::new();
        println!(
            "What unit would you like to convert {} (F/C/K/R)?",
            if use_from { "from" } else { "to" }
        );

        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.to_uppercase().trim() {
                "F" => return Some(TemperatureUnit::Fahrenheit),
                "C" => return Some(TemperatureUnit::Celsius),
                "K" => return Some(TemperatureUnit::Kelvin),
                "R" => return Some(TemperatureUnit::Rankine),
                "QUIT" => return None,
                _ => { 
                    println!("Your input must be one of F,C,K,R. Please type \"quit\" to exit."); 
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