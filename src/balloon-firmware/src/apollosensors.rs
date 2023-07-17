extern crate open_location_code;
extern crate rand;

use alloc::string::String;
use rand::distributions::uniform::Uniform;
use rand::prelude::Distribution;
use rand::SeedableRng;
use rand::rngs::SmallRng;

use crate::parameters::OLC_CODE_LENGTH;

// Sensor-related functions (to be replaced with actual sensor code)

pub fn get_location() -> ([u8; OLC_CODE_LENGTH], [u8; 4], [u8; 4]) {

    
    // Returns a tuple containing the Open Location Code, latitude, and longitude
    // of the balloon.
    // TODO: Replace with actual sensor code.
    let mut rng = SmallRng::from_entropy();

    // Create a uniform distribution between -90 and 90 degrees.
    let latitude_range: Uniform<f32> = Uniform::new(-90.0, 90.0);

    // Create a uniform distribution between -180 and 180 degrees.
    let longitude_range: Uniform<f32> = Uniform::new(-180.0, 180.0);

    // Generate a random latitude and longitude.
    let latitude: f32 = latitude_range.sample(&mut rng);
    let longitude: f32 = longitude_range.sample(&mut rng);

    let point = geo_types::Point::new(latitude as f64, longitude as f64);
    // Populate the Open Location Code string.
    let mut olc_code: String = open_location_code::encode(point, OLC_CODE_LENGTH);

    let olc_code_str: &str = olc_code.as_mut_str();
    let mut olc_code_bytes: [u8; OLC_CODE_LENGTH] = [0; OLC_CODE_LENGTH];
    olc_code_bytes.copy_from_slice(olc_code_str.as_bytes());
    let latitude_bytes = latitude.to_be_bytes();
    let longitude_bytes = longitude.to_be_bytes();
    return (olc_code_bytes, latitude_bytes, longitude_bytes);
}

pub fn get_altitude() -> [u8; 4] {
    // Returns the altitude of the balloon, in meters.
    // TODO: Replace with actual sensor code.

    // Generate a random altitude between 0 and 10,000 meters.
    let mut rng = rand::thread_rng();
    let altitude_range = Uniform::new(0.0, 10000.0);
    let altitude: f32 = altitude_range.sample(&mut rng);
    return altitude.to_be_bytes();
}

pub fn get_voltage() -> [u8; 4] {
    // Returns the voltage difference from nomal voltage, in volts.
    // TODO: Replace with actual sensor code.

    // Generate a random voltage difference between -0.1 and 1.5 volts.
    let mut rng = rand::thread_rng();
    let voltage_range = Uniform::new(-0.1, 1.5);
    let voltage: f32 = voltage_range.sample(&mut rng);
    return voltage.to_be_bytes();
}

pub fn get_temperature() -> [u8; 4] {
    // Returns the temperature of the balloon, in degrees Celsius.
    // TODO: Replace with actual sensor code.

    // Generate a random temperature between -50 and 50 degrees Celsius.
    let mut rng = rand::thread_rng();
    let temperature_range = Uniform::new(-50.0, 50.0);
    let temperature: f32 = temperature_range.sample(&mut rng);
    return temperature.to_be_bytes();
}