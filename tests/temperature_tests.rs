// temperature_tests.rs

use TemperatureConverter::{
    Temperature, 
    TemperatureUnit
};

// Constants
const FAHRENHEIT_FREEZING_TEMP: f64 = 32.0;
const CELSIUS_FREEZING_TEMP: f64 = 0.0;
const KELVIN_FREEZING_TEMP: f64 = 273.15;
const RANKINE_FREEZING_TEMP: f64 = 491.67;
const DELISLE_FREEZING_TEMP: f64 = 150.0;
const NEWTON_FREEZING_TEMP: f64 = 0.0;
const REAUMUR_FREEZING_TEMP: f64 = 0.0;
const ROMER_FREEZING_TEMP: f64 = 7.5;

const FAHRENHEIT_BOILING_TEMP: f64 = 212.0;
const CELSIUS_BOILING_TEMP: f64 = 100.0;
const KELVIN_BOILING_TEMP: f64 = 373.15;
const RANKINE_BOILING_TEMP: f64 = 671.67;
const DELISLE_BOILING_TEMP: f64 = 0.0;
const NEWTON_BOILING_TEMP: f64 = 33.0;
const REAUMUR_BOILING_TEMP: f64 = 80.0;
const ROMER_BOILING_TEMP: f64 = 60.0;

const EPSILON: f64 = 1.0e-12;

fn approximately_equal(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() <= epsilon
}

//
//
// Freezing point conversion tests
//
//

#[test]
fn test_fahrenheit_to_other_freezing_point_conversions() {
    // Freezing point in Fahrenheit
    let fahrenheit = Temperature::new(FAHRENHEIT_FREEZING_TEMP, TemperatureUnit::Fahrenheit);

    let celsius = fahrenheit.convert_to(TemperatureUnit::Celsius);
    assert!(approximately_equal(celsius.value, CELSIUS_FREEZING_TEMP, EPSILON));

    let kelvin = fahrenheit.convert_to(TemperatureUnit::Kelvin);
    assert!(approximately_equal(kelvin.value, KELVIN_FREEZING_TEMP, EPSILON));

    let rankine = fahrenheit.convert_to(TemperatureUnit::Rankine);
    assert!(approximately_equal(rankine.value, RANKINE_FREEZING_TEMP, EPSILON));

    let delisle = fahrenheit.convert_to(TemperatureUnit::Delisle);
    assert!(approximately_equal(delisle.value, DELISLE_FREEZING_TEMP, EPSILON));

    let newton = fahrenheit.convert_to(TemperatureUnit::Newton);
    assert!(approximately_equal(newton.value, NEWTON_FREEZING_TEMP, EPSILON));
   
   let reaumur = fahrenheit.convert_to(TemperatureUnit::Reaumur);
   assert!(approximately_equal(reaumur.value, REAUMUR_FREEZING_TEMP, EPSILON));

   let romer = fahrenheit.convert_to(TemperatureUnit::Romer);
   assert!(approximately_equal(romer.value,ROMER_FREEZING_TEMP, EPSILON));
}

#[test]
fn test_celsius_to_other_freezing_point_conversions() {
    // Freezing point in Celsius
    let celsius = Temperature::new(CELSIUS_FREEZING_TEMP, TemperatureUnit::Celsius);

    let fahrenheit = celsius.convert_to(TemperatureUnit::Fahrenheit);
    assert!(approximately_equal(fahrenheit.value, FAHRENHEIT_FREEZING_TEMP, EPSILON));

    let kelvin = celsius.convert_to(TemperatureUnit::Kelvin);
    assert!(approximately_equal(kelvin.value, KELVIN_FREEZING_TEMP, EPSILON));

    let rankine = celsius.convert_to(TemperatureUnit::Rankine);
    assert!(approximately_equal(rankine.value, RANKINE_FREEZING_TEMP, EPSILON));

    let delisle = celsius.convert_to(TemperatureUnit::Delisle);
    assert!(approximately_equal(delisle.value, DELISLE_FREEZING_TEMP, EPSILON));

    let newton = celsius.convert_to(TemperatureUnit::Newton);
    assert!(approximately_equal(newton.value, NEWTON_FREEZING_TEMP, EPSILON));
   
    let reaumur = celsius.convert_to(TemperatureUnit::Reaumur);
    assert!(approximately_equal(reaumur.value, REAUMUR_FREEZING_TEMP, EPSILON));
 
    let romer = celsius.convert_to(TemperatureUnit::Romer);
    assert!(approximately_equal(romer.value,ROMER_FREEZING_TEMP, EPSILON));
 }

#[test]
fn test_kelvin_to_other_freezing_point_conversions() {
    // Freezing point in Kelvin
    let kelvin = Temperature::new(KELVIN_FREEZING_TEMP, TemperatureUnit::Kelvin);

    let fahrenheit = kelvin.convert_to(TemperatureUnit::Fahrenheit);
    assert!(approximately_equal(fahrenheit.value, FAHRENHEIT_FREEZING_TEMP, EPSILON));

    let celsius = kelvin.convert_to(TemperatureUnit::Celsius);
    assert!(approximately_equal(celsius.value, CELSIUS_FREEZING_TEMP, EPSILON));

    let rankine = kelvin.convert_to(TemperatureUnit::Rankine);
    assert!(approximately_equal(rankine.value, RANKINE_FREEZING_TEMP, EPSILON));

    let delisle = kelvin.convert_to(TemperatureUnit::Delisle);
    assert!(approximately_equal(delisle.value, DELISLE_FREEZING_TEMP, EPSILON));

    let newton = kelvin.convert_to(TemperatureUnit::Newton);
    assert!(approximately_equal(newton.value, NEWTON_FREEZING_TEMP, EPSILON));
   
    let reaumur = kelvin.convert_to(TemperatureUnit::Reaumur);
    assert!(approximately_equal(reaumur.value, REAUMUR_FREEZING_TEMP, EPSILON));
 
    let romer = kelvin.convert_to(TemperatureUnit::Romer);
    assert!(approximately_equal(romer.value,ROMER_FREEZING_TEMP, EPSILON));
 }

#[test]
fn test_rankine_to_other_freezing_point_conversions() {
    // Freezing point in Rankine
    let rankine = Temperature::new(RANKINE_FREEZING_TEMP, TemperatureUnit::Rankine);

    let fahrenheit = rankine.convert_to(TemperatureUnit::Fahrenheit);
    assert!(approximately_equal(fahrenheit.value, FAHRENHEIT_FREEZING_TEMP, EPSILON));

    let celsius = rankine.convert_to(TemperatureUnit::Celsius);
    assert!(approximately_equal(celsius.value, CELSIUS_FREEZING_TEMP, EPSILON));

    let kelvin = rankine.convert_to(TemperatureUnit::Kelvin);
    assert!(approximately_equal(kelvin.value, KELVIN_FREEZING_TEMP, EPSILON));

    let delisle = rankine.convert_to(TemperatureUnit::Delisle);
    assert!(approximately_equal(delisle.value, DELISLE_FREEZING_TEMP, EPSILON));

    let newton = rankine.convert_to(TemperatureUnit::Newton);
    assert!(approximately_equal(newton.value, NEWTON_FREEZING_TEMP, EPSILON));
   
    let reaumur = rankine.convert_to(TemperatureUnit::Reaumur);
    assert!(approximately_equal(reaumur.value, REAUMUR_FREEZING_TEMP, EPSILON));
 
    let romer = rankine.convert_to(TemperatureUnit::Romer);
    assert!(approximately_equal(romer.value,ROMER_FREEZING_TEMP, EPSILON));
 }

#[test]
fn test_delisle_to_other_freezing_point_conversions() {
    // Freezing point in Delisle
    let delisle = Temperature::new(DELISLE_FREEZING_TEMP, TemperatureUnit::Delisle);

    let fahrenheit = delisle.convert_to(TemperatureUnit::Fahrenheit);
    assert!(approximately_equal(fahrenheit.value, FAHRENHEIT_FREEZING_TEMP, EPSILON));

    let celsius = delisle.convert_to(TemperatureUnit::Celsius);
    assert!(approximately_equal(celsius.value, CELSIUS_FREEZING_TEMP, EPSILON));

    let kelvin = delisle.convert_to(TemperatureUnit::Kelvin);
    assert!(approximately_equal(kelvin.value, KELVIN_FREEZING_TEMP, EPSILON));

    let rankine = delisle.convert_to(TemperatureUnit::Rankine);
    assert!(approximately_equal(rankine.value, RANKINE_FREEZING_TEMP, EPSILON));

    let newton = delisle.convert_to(TemperatureUnit::Newton);
    assert!(approximately_equal(newton.value, NEWTON_FREEZING_TEMP, EPSILON));
   
    let reaumur = delisle.convert_to(TemperatureUnit::Reaumur);
    assert!(approximately_equal(reaumur.value, REAUMUR_FREEZING_TEMP, EPSILON));
 
    let romer = delisle.convert_to(TemperatureUnit::Romer);
    assert!(approximately_equal(romer.value,ROMER_FREEZING_TEMP, EPSILON));
 }

#[test]
fn test_newton_to_other_freezing_point_conversions() {
    // Freezing point in Newton
    let newton = Temperature::new(NEWTON_FREEZING_TEMP, TemperatureUnit::Newton);

    let fahrenheit = newton.convert_to(TemperatureUnit::Fahrenheit);
    assert!(approximately_equal(fahrenheit.value, FAHRENHEIT_FREEZING_TEMP, EPSILON));

    let celsius = newton.convert_to(TemperatureUnit::Celsius);
    assert!(approximately_equal(celsius.value, CELSIUS_FREEZING_TEMP, EPSILON));

    let kelvin = newton.convert_to(TemperatureUnit::Kelvin);
    assert!(approximately_equal(kelvin.value, KELVIN_FREEZING_TEMP, EPSILON));

    let rankine = newton.convert_to(TemperatureUnit::Rankine);
    assert!(approximately_equal(rankine.value, RANKINE_FREEZING_TEMP, EPSILON));

    let delisle = newton.convert_to(TemperatureUnit::Delisle);
    assert!(approximately_equal(delisle.value, DELISLE_FREEZING_TEMP, EPSILON));
   
    let reaumur = newton.convert_to(TemperatureUnit::Reaumur);
    assert!(approximately_equal(reaumur.value, REAUMUR_FREEZING_TEMP, EPSILON));
 
    let romer = newton.convert_to(TemperatureUnit::Romer);
    assert!(approximately_equal(romer.value,ROMER_FREEZING_TEMP, EPSILON));
 }

 #[test]
 fn test_reaumur_to_other_freezing_point_conversions() {
    // Freezing point in Reaumur  
    let reaumur = Temperature::new(REAUMUR_FREEZING_TEMP, TemperatureUnit::Reaumur);
    
    let fahrenheit = reaumur.convert_to(TemperatureUnit::Fahrenheit);
    assert!(approximately_equal(fahrenheit.value, FAHRENHEIT_FREEZING_TEMP, EPSILON));

    let celsius = reaumur.convert_to(TemperatureUnit::Celsius);
    assert!(approximately_equal(celsius.value, CELSIUS_FREEZING_TEMP, EPSILON));

    let kelvin = reaumur.convert_to(TemperatureUnit::Kelvin);
    assert!(approximately_equal(kelvin.value, KELVIN_FREEZING_TEMP, EPSILON));

    let rankine = reaumur.convert_to(TemperatureUnit::Rankine);
    assert!(approximately_equal(rankine.value, RANKINE_FREEZING_TEMP, EPSILON));

    let delisle = reaumur.convert_to(TemperatureUnit::Delisle);
    assert!(approximately_equal(delisle.value, DELISLE_FREEZING_TEMP, EPSILON));
   
    let newton = reaumur.convert_to(TemperatureUnit::Newton);
    assert!(approximately_equal(newton.value, NEWTON_FREEZING_TEMP, EPSILON));

    let romer = reaumur.convert_to(TemperatureUnit::Romer);
    assert!(approximately_equal(romer.value,ROMER_FREEZING_TEMP, EPSILON));
 }

 #[test]
fn test_romer_to_other_freezing_point_conversions() {
    // Freezing point in Romer
    let romer = Temperature::new(ROMER_FREEZING_TEMP, TemperatureUnit::Romer);

    let fahrenheit = romer.convert_to(TemperatureUnit::Fahrenheit);
    assert!(approximately_equal(fahrenheit.value, FAHRENHEIT_FREEZING_TEMP, EPSILON));

    let celsius = romer.convert_to(TemperatureUnit::Celsius);
    assert!(approximately_equal(celsius.value, CELSIUS_FREEZING_TEMP, EPSILON));

    let kelvin = romer.convert_to(TemperatureUnit::Kelvin);
    assert!(approximately_equal(kelvin.value, KELVIN_FREEZING_TEMP, EPSILON));

    let rankine = romer.convert_to(TemperatureUnit::Rankine);
    assert!(approximately_equal(rankine.value, RANKINE_FREEZING_TEMP, EPSILON));

    let delisle = romer.convert_to(TemperatureUnit::Delisle);
    assert!(approximately_equal(delisle.value, DELISLE_FREEZING_TEMP, EPSILON));

    let newton = romer.convert_to(TemperatureUnit::Newton);
    assert!(approximately_equal(newton.value, NEWTON_FREEZING_TEMP, EPSILON));

    let reaumur = romer.convert_to(TemperatureUnit::Reaumur);
    assert!(approximately_equal(reaumur.value, REAUMUR_FREEZING_TEMP, EPSILON));
}
//
//
// Boiling point conversion tests
//
//

#[test]
fn test_fahrenheit_to_other_boiling_point_conversions() {
    // Boiling point in Fahrenheit
    let fahrenheit = Temperature::new(FAHRENHEIT_BOILING_TEMP, TemperatureUnit::Fahrenheit);

    let celsius = fahrenheit.convert_to(TemperatureUnit::Celsius);
    assert!(approximately_equal(celsius.value, CELSIUS_BOILING_TEMP, EPSILON));

    let kelvin = fahrenheit.convert_to(TemperatureUnit::Kelvin);
    assert!(approximately_equal(kelvin.value, KELVIN_BOILING_TEMP, EPSILON));

    let rankine = fahrenheit.convert_to(TemperatureUnit::Rankine);
    assert!(approximately_equal(rankine.value, RANKINE_BOILING_TEMP, EPSILON));

    let delisle = fahrenheit.convert_to(TemperatureUnit::Delisle);
    assert!(approximately_equal(delisle.value, DELISLE_BOILING_TEMP, EPSILON));

    let newton = fahrenheit.convert_to(TemperatureUnit::Newton);
    assert!(approximately_equal(newton.value, NEWTON_BOILING_TEMP, EPSILON));
    
    let reaumur = fahrenheit.convert_to(TemperatureUnit::Reaumur);
    assert!(approximately_equal(reaumur.value, REAUMUR_BOILING_TEMP, EPSILON));
    
    let romer = fahrenheit.convert_to(TemperatureUnit::Romer);
    assert!(approximately_equal(romer.value, ROMER_BOILING_TEMP, EPSILON));
}

#[test]
fn test_celsius_to_other_boiling_point_conversions() {
    // Boiling point in Celsius
    let celsius = Temperature::new(CELSIUS_BOILING_TEMP, TemperatureUnit::Celsius);

    let fahrenheit = celsius.convert_to(TemperatureUnit::Fahrenheit);
    assert!(approximately_equal(fahrenheit.value, FAHRENHEIT_BOILING_TEMP, EPSILON));

    let kelvin = celsius.convert_to(TemperatureUnit::Kelvin);
    assert!(approximately_equal(kelvin.value, KELVIN_BOILING_TEMP, EPSILON));

    let rankine = celsius.convert_to(TemperatureUnit::Rankine);
    assert!(approximately_equal(rankine.value, RANKINE_BOILING_TEMP, EPSILON));

    let delisle = celsius.convert_to(TemperatureUnit::Delisle);
    assert!(approximately_equal(delisle.value, DELISLE_BOILING_TEMP, EPSILON));

    let newton = celsius.convert_to(TemperatureUnit::Newton);
    assert!(approximately_equal(newton.value, NEWTON_BOILING_TEMP, EPSILON));
    
    let reaumur = celsius.convert_to(TemperatureUnit::Reaumur);
    assert!(approximately_equal(reaumur.value, REAUMUR_BOILING_TEMP, EPSILON));
    
    let romer = celsius.convert_to(TemperatureUnit::Romer);
    assert!(approximately_equal(romer.value, ROMER_BOILING_TEMP, EPSILON));
}

#[test]
fn test_kelvin_to_other_boiling_point_conversions() {
    // Boiling point in Kelvin
    let kelvin = Temperature::new(KELVIN_BOILING_TEMP, TemperatureUnit::Kelvin);

    let fahrenheit = kelvin.convert_to(TemperatureUnit::Fahrenheit);
    assert!(approximately_equal(fahrenheit.value, FAHRENHEIT_BOILING_TEMP, EPSILON));

    let celsius = kelvin.convert_to(TemperatureUnit::Celsius);
    assert!(approximately_equal(celsius.value, CELSIUS_BOILING_TEMP, EPSILON));

    let rankine = kelvin.convert_to(TemperatureUnit::Rankine);
    assert!(approximately_equal(rankine.value, RANKINE_BOILING_TEMP, EPSILON));

    let delisle = kelvin.convert_to(TemperatureUnit::Delisle);
    assert!(approximately_equal(delisle.value, DELISLE_BOILING_TEMP, EPSILON));

    let newton = kelvin.convert_to(TemperatureUnit::Newton);
    assert!(approximately_equal(newton.value, NEWTON_BOILING_TEMP, EPSILON));
    
    let reaumur = kelvin.convert_to(TemperatureUnit::Reaumur);
    assert!(approximately_equal(reaumur.value, REAUMUR_BOILING_TEMP, EPSILON));
    
    let romer = kelvin.convert_to(TemperatureUnit::Romer);
    assert!(approximately_equal(romer.value, ROMER_BOILING_TEMP, EPSILON));
}

#[test]
fn test_rankine_to_other_boiling_point_conversions() {
    // Boiling point in Rankine
    let rankine = Temperature::new(RANKINE_BOILING_TEMP, TemperatureUnit::Rankine);

    let fahrenheit = rankine.convert_to(TemperatureUnit::Fahrenheit);
    assert!(approximately_equal(fahrenheit.value, FAHRENHEIT_BOILING_TEMP, EPSILON));

    let celsius = rankine.convert_to(TemperatureUnit::Celsius);
    assert!(approximately_equal(celsius.value, CELSIUS_BOILING_TEMP, EPSILON));

    let kelvin = rankine.convert_to(TemperatureUnit::Kelvin);
    assert!(approximately_equal(kelvin.value, KELVIN_BOILING_TEMP, EPSILON));

    let delisle = rankine.convert_to(TemperatureUnit::Delisle);
    assert!(approximately_equal(delisle.value, DELISLE_BOILING_TEMP, EPSILON));

    let newton = rankine.convert_to(TemperatureUnit::Newton);
    assert!(approximately_equal(newton.value, NEWTON_BOILING_TEMP, EPSILON));
    
    let reaumur = rankine.convert_to(TemperatureUnit::Reaumur);
    assert!(approximately_equal(reaumur.value, REAUMUR_BOILING_TEMP, EPSILON));
    
    let romer = rankine.convert_to(TemperatureUnit::Romer);
    assert!(approximately_equal(romer.value, ROMER_BOILING_TEMP, EPSILON));
}

#[test]
fn test_delisle_to_other_boiling_point_conversions() {
    // Boiling point in Delisle
    let delisle = Temperature::new(DELISLE_BOILING_TEMP, TemperatureUnit::Delisle);

    let fahrenheit = delisle.convert_to(TemperatureUnit::Fahrenheit);
    assert!(approximately_equal(fahrenheit.value, FAHRENHEIT_BOILING_TEMP, EPSILON));

    let celsius = delisle.convert_to(TemperatureUnit::Celsius);
    assert!(approximately_equal(celsius.value, CELSIUS_BOILING_TEMP, EPSILON));

    let kelvin = delisle.convert_to(TemperatureUnit::Kelvin);
    assert!(approximately_equal(kelvin.value, KELVIN_BOILING_TEMP, EPSILON));

    let rankine = delisle.convert_to(TemperatureUnit::Rankine);
    assert!(approximately_equal(rankine.value, RANKINE_BOILING_TEMP, EPSILON));

    let newton = delisle.convert_to(TemperatureUnit::Newton);
    assert!(approximately_equal(newton.value, NEWTON_BOILING_TEMP, EPSILON));
    
    let reaumur = delisle.convert_to(TemperatureUnit::Reaumur);
    assert!(approximately_equal(reaumur.value, REAUMUR_BOILING_TEMP, EPSILON));
    
    let romer = delisle.convert_to(TemperatureUnit::Romer);
    assert!(approximately_equal(romer.value, ROMER_BOILING_TEMP, EPSILON));
}

#[test]
fn test_newton_to_other_boiling_point_conversions() {
    // Boiling point in Newton
    let newton = Temperature::new(NEWTON_BOILING_TEMP, TemperatureUnit::Newton);

    let fahrenheit = newton.convert_to(TemperatureUnit::Fahrenheit);
    assert!(approximately_equal(fahrenheit.value, FAHRENHEIT_BOILING_TEMP, EPSILON));

    let celsius = newton.convert_to(TemperatureUnit::Celsius);
    assert!(approximately_equal(celsius.value, CELSIUS_BOILING_TEMP, EPSILON));

    let kelvin = newton.convert_to(TemperatureUnit::Kelvin);
    assert!(approximately_equal(kelvin.value, KELVIN_BOILING_TEMP, EPSILON));

    let rankine = newton.convert_to(TemperatureUnit::Rankine);
    assert!(approximately_equal(rankine.value, RANKINE_BOILING_TEMP, EPSILON));

    let delisle = newton.convert_to(TemperatureUnit::Delisle);
    assert!(approximately_equal(delisle.value, DELISLE_BOILING_TEMP, EPSILON));
    
    let reaumur = newton.convert_to(TemperatureUnit::Reaumur);
    assert!(approximately_equal(reaumur.value, REAUMUR_BOILING_TEMP, EPSILON));
    
    let romer = newton.convert_to(TemperatureUnit::Romer);
    assert!(approximately_equal(romer.value, ROMER_BOILING_TEMP, EPSILON));
}

#[test]
fn test_reaumur_to_other_boiling_point_conversions() {
    // Boiling point in Reaumur
    let reaumur = Temperature::new(REAUMUR_BOILING_TEMP, TemperatureUnit::Reaumur); 

    let fahrenheit = reaumur.convert_to(TemperatureUnit::Fahrenheit);
    assert!(approximately_equal(fahrenheit.value, FAHRENHEIT_BOILING_TEMP, EPSILON));

    let celsius = reaumur.convert_to(TemperatureUnit::Celsius);
    println!("Expected {} Actual {}", CELSIUS_BOILING_TEMP, celsius.value);
    assert!(approximately_equal(celsius.value, CELSIUS_BOILING_TEMP, EPSILON));

    let kelvin = reaumur.convert_to(TemperatureUnit::Kelvin);
    assert!(approximately_equal(kelvin.value, KELVIN_BOILING_TEMP, EPSILON));

    let rankine = reaumur.convert_to(TemperatureUnit::Rankine);
    assert!(approximately_equal(rankine.value, RANKINE_BOILING_TEMP, EPSILON));

    let delisle = reaumur.convert_to(TemperatureUnit::Delisle);
    assert!(approximately_equal(delisle.value, DELISLE_BOILING_TEMP, EPSILON));
  
    let newton = reaumur.convert_to(TemperatureUnit::Newton);
    assert!(approximately_equal(newton.value, NEWTON_BOILING_TEMP, EPSILON));
    
    let romer = reaumur.convert_to(TemperatureUnit::Romer);
    assert!(approximately_equal(romer.value, ROMER_BOILING_TEMP, EPSILON));
}

#[test]
fn test_romer_to_other_boiling_point_conversions() {
    // Boiling point in Romer
    let romer = Temperature::new(ROMER_BOILING_TEMP, TemperatureUnit::Romer);

    let fahrenheit = romer.convert_to(TemperatureUnit::Fahrenheit);
    assert!(approximately_equal(fahrenheit.value, FAHRENHEIT_BOILING_TEMP, EPSILON));

    let celsius = romer.convert_to(TemperatureUnit::Celsius);
    assert!(approximately_equal(celsius.value, CELSIUS_BOILING_TEMP, EPSILON));

    let kelvin = romer.convert_to(TemperatureUnit::Kelvin);
    assert!(approximately_equal(kelvin.value, KELVIN_BOILING_TEMP, EPSILON));

    let rankine = romer.convert_to(TemperatureUnit::Rankine);
    assert!(approximately_equal(rankine.value, RANKINE_BOILING_TEMP, EPSILON));

    let delisle = romer.convert_to(TemperatureUnit::Delisle);
    assert!(approximately_equal(delisle.value, DELISLE_BOILING_TEMP, EPSILON));

    let newton = romer.convert_to(TemperatureUnit::Newton);
    assert!(approximately_equal(newton.value, NEWTON_BOILING_TEMP, EPSILON));

    let reaumur = romer.convert_to(TemperatureUnit::Reaumur);
    assert!(approximately_equal(reaumur.value, REAUMUR_BOILING_TEMP, EPSILON));
}