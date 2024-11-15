// temperature_tests.rs

use TemperatureConverter::{
    Temperature, 
    TemperatureUnit,

    FAHRENHEIT_FREEZING_TEMP,
    CELSIUS_FREEZING_TEMP,
    KELVIN_FREEZING_TEMP,
    RANKINE_FREEZING_TEMP,
    DELISLE_FREEZING_TEMP,
    NEWTON_FREEZING_TEMP,

    FAHRENHEIT_BOILING_TEMP,
    CELSIUS_BOILING_TEMP,
    KELVIN_BOILING_TEMP,
    RANKINE_BOILING_TEMP,
    DELISLE_BOILING_TEMP,
    NEWTON_BOILING_TEMP,
};

const EPSILON: f64 = 1.0e-12;

fn approximately_equal(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() <= epsilon
}

// Freezing point conversions

#[test]
fn test_fahrenheit_to_other_freezing_point_conversions() {
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
}

#[test]
fn test_celsius_to_other_freezing_point_conversions() {
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

}

#[test]
fn test_kelvin_to_other_freezing_point_conversions() {
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
}

#[test]
fn test_rankine_to_other_freezing_point_conversions() {
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
}

#[test]
fn test_delisle_to_other_freezing_point_conversions() {
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
}

#[test]
fn test_newton_to_other_freezing_point_conversions() {
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
}


// Boiling point conversions

#[test]
fn test_fahrenheit_to_other_boiling_point_conversions() {
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
}

#[test]
fn test_celsius_to_other_boiling_point_conversions() {
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
}

#[test]
fn test_kelvin_to_other_boiling_point_conversions() {
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
}

#[test]
fn test_rankine_to_other_boiling_point_conversions() {
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
}

#[test]
fn test_delisle_to_other_boiling_point_conversions() {
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
}

#[test]
fn test_newton_to_other_boiling_point_conversions() {
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
}
