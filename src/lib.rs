// lib.rs

pub const FAHRENHEIT_FREEZING_TEMP: f64 = 32.0;
pub const CELSIUS_FREEZING_TEMP: f64 = 0.0;
pub const KELVIN_FREEZING_TEMP: f64 = 273.15;
pub const RANKINE_FREEZING_TEMP: f64 = 491.67;
pub const DELISLE_FREEZING_TEMP: f64 = 150.0;
pub const NEWTON_FREEZING_TEMP: f64 = 0.0;

pub const FAHRENHEIT_BOILING_TEMP: f64 = 212.0;
pub const CELSIUS_BOILING_TEMP: f64 = 100.0;
pub const KELVIN_BOILING_TEMP: f64 = 373.15;
pub const RANKINE_BOILING_TEMP: f64 = 671.67;
pub const DELISLE_BOILING_TEMP: f64 = 0.0;
pub const NEWTON_BOILING_TEMP: f64 = 33.0;

const DEGREE_SYMBOL: &str = "\u{00B0}";
const LOWERCASE_E_ACUTE_ACCENT: &str = "\u{00E9}";

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TemperatureUnit {
    Fahrenheit,
    Celsius,
    Kelvin,
    Rankine,
    Delisle,
    Newton,
}

pub const TEMP_OPTIONS: &str = "F/C/K/R/De/Re";

impl TemperatureUnit {
    pub fn to_string(self) -> String {
        match self {
            TemperatureUnit::Fahrenheit => format!("{DEGREE_SYMBOL}F"),
            TemperatureUnit::Celsius => format!("{DEGREE_SYMBOL}C"),
            TemperatureUnit::Kelvin => format!("K"),
            TemperatureUnit::Rankine => format!("{DEGREE_SYMBOL}R"),
            TemperatureUnit::Delisle => format!("{DEGREE_SYMBOL}De"),
            TemperatureUnit::Newton => format!("{DEGREE_SYMBOL}R{LOWERCASE_E_ACUTE_ACCENT}"),
        }
    }
}

pub struct Temperature {
    pub value: f64,
    pub unit: TemperatureUnit,
}

impl Temperature {
    pub fn new(value: f64, unit: TemperatureUnit) -> Self {
        Temperature { value, unit }
    }

    pub fn convert_to(&self, target_unit: TemperatureUnit) -> Temperature {
        let value_in_target_unit = match (self.unit, target_unit) {
            (TemperatureUnit::Fahrenheit, TemperatureUnit::Celsius) => (self.value - 32.0) / 1.8,
            (TemperatureUnit::Fahrenheit, TemperatureUnit::Kelvin) => ((self.value - 32.0) / 1.8) + 273.15,
            (TemperatureUnit::Fahrenheit, TemperatureUnit::Rankine) => self.value + 459.67,
            (TemperatureUnit::Fahrenheit, TemperatureUnit::Delisle) => ((212.0 - self.value) / 6.0) * 5.0,
            (TemperatureUnit::Fahrenheit, TemperatureUnit::Newton) => ((self.value - 32.0) / 60.0) * 11.0,

            (TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit) => (self.value * 1.8) + 32.0,
            (TemperatureUnit::Celsius, TemperatureUnit::Kelvin) => self.value + 273.15,
            (TemperatureUnit::Celsius, TemperatureUnit::Rankine) => (self.value * 1.8) + 491.67,
            (TemperatureUnit::Celsius, TemperatureUnit::Delisle) => ((100.0 - self.value) / 2.0) * 3.0,
            (TemperatureUnit::Celsius, TemperatureUnit::Newton) => (self.value / 100.0) * 33.0,

            (TemperatureUnit::Kelvin, TemperatureUnit::Fahrenheit) => ((self.value - 273.15) * 1.8) + 32.0,
            (TemperatureUnit::Kelvin, TemperatureUnit::Celsius) => self.value - 273.15,
            (TemperatureUnit::Kelvin, TemperatureUnit::Rankine) =>  ((self.value - 273.15) * 1.8) + 491.67,
            (TemperatureUnit::Kelvin, TemperatureUnit::Delisle) => ((373.15 - self.value ) / 2.0) * 3.0,
            (TemperatureUnit::Kelvin, TemperatureUnit::Newton) => ((self.value - 273.15) / 100.0) * 33.0,

            (TemperatureUnit::Rankine, TemperatureUnit::Fahrenheit) => self.value - 459.67,
            (TemperatureUnit::Rankine, TemperatureUnit::Celsius) => (self.value - 491.67) / 1.8,
            (TemperatureUnit::Rankine, TemperatureUnit::Kelvin) =>  ((self.value - 491.67) / 1.8) + 273.15,
            (TemperatureUnit::Rankine, TemperatureUnit::Delisle) => ((671.67 - self.value) / 6.0) * 5.0,
            (TemperatureUnit::Rankine, TemperatureUnit::Newton) => ((self.value - 491.67) / 60.0) * 11.0,

            (TemperatureUnit::Delisle, TemperatureUnit::Fahrenheit) => 212.0 - ((self.value / 5.0) * 6.0),
            (TemperatureUnit::Delisle, TemperatureUnit::Celsius) => 100.0 - ((self.value / 3.0) * 2.0),
            (TemperatureUnit::Delisle, TemperatureUnit::Kelvin) => 373.15 - ((self.value / 3.0) * 2.0),
            (TemperatureUnit::Delisle, TemperatureUnit::Rankine) => 671.67 - ((self.value / 5.0) * 6.0),
            (TemperatureUnit::Delisle, TemperatureUnit::Newton) => 33.0 - ((self.value / 50.0) * 11.0),

            (TemperatureUnit::Newton, TemperatureUnit::Fahrenheit) => ((self.value / 11.0) * 60.0) + 32.0,
            (TemperatureUnit::Newton, TemperatureUnit::Celsius) => (self.value / 33.0) * 100.0,
            (TemperatureUnit::Newton, TemperatureUnit::Kelvin) => ((self.value / 33.0) * 100.0) + 273.15,
            (TemperatureUnit::Newton, TemperatureUnit::Rankine) => ((self.value / 11.0) * 60.0) + 491.67,
            (TemperatureUnit::Newton, TemperatureUnit::Delisle) => ((33.0 - self.value) / 11.0) * 50.0,


            // To & from the same unit
            (_, _) => self.value,
        };

        if value_in_target_unit.is_infinite() {
            panic!("Temperature.convert_to() encountered <inf>");
        }
        else if value_in_target_unit.is_nan() {
            panic!("Temperature.convert_to() encountered <NaN>");
        }

        Temperature::new(value_in_target_unit, target_unit)
    }

    pub fn to_string(&self) -> String {
        format!("{}{}", format_with_commas(self.value), self.unit.to_string())
    }
}

pub fn format_with_commas(number: f64) -> String {
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