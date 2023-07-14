#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![no_std]
// #![no_main]

pub mod parameters;
pub mod apollosensors;
pub mod apollotelemetry;
use core::default;

use libc_print::std_name::{println, eprintln, dbg};

fn main() {
    unsafe { // TODO: horrific
        pub static mut _olc_code: [u8; parameters::OLC_CODE_LENGTH - 1] = [0; parameters::OLC_CODE_LENGTH - 1];
        pub static mut _latitude: [u8; 4] = [0u8; 4];
        pub static mut _longitude: [u8; 4] = [0u8; 4];
        pub static mut _altitude: [u8; 4] = [0u8; 4];
        pub static mut _voltage: [u8; 4] = [0u8; 4];
        pub static mut _temperature: [u8; 4] = [0u8; 4];

        (_olc_code, _latitude, _longitude) = apollosensors::get_location();
        _altitude = apollosensors::get_altitude();
        _voltage = apollosensors::get_voltage();
        _temperature = apollosensors::get_temperature();

        // quick testing values

        _latitude = 69.1337f32.to_be_bytes();
        _longitude = 420.1337f32.to_be_bytes();
        _altitude = 1337.69f32.to_be_bytes();
        _voltage = 420.69f32.to_be_bytes();
        _temperature = 69.420f32.to_be_bytes();

        let mut block_data = [0u8; 4];
        
        let mut _blocks = apollotelemetry::construct_blocks(&_olc_code, &_altitude, &_voltage, &_temperature, &_latitude, &_longitude);
        if cfg!(debug_assertions) { // this doesn't work right now for some reason
            for _block in _blocks.iter() {
                match _block.name{
                    "Start Header" | "End Header" => {
                        println!("{}: {}", _block.name, core::str::from_utf8(&_block.data).unwrap());
                    },
                    "OLC Code" => {
                        println!("{}: {}", _block.name, core::str::from_utf8(&_block.data).unwrap());
                    },
                    "Altitude" => {
                        block_data.copy_from_slice(&_block.data);
                        println!("{}: {}", _block.name, f32::from_be_bytes(block_data));
                    },
                    "Battery Voltage" => {
                        block_data.copy_from_slice(&_block.data);
                        println!("{}: {}", _block.name, f32::from_be_bytes(block_data));
                    },
                    "Temperature" => {
                        block_data.copy_from_slice(&_block.data);
                        println!("{}: {}", _block.name, f32::from_be_bytes(block_data));
                    },
                    "Latitude" => {
                        block_data.copy_from_slice(&_block.data);
                        println!("{}: {}", _block.name, f32::from_be_bytes(block_data));
                    },
                    "Longitude" => {
                        block_data.copy_from_slice(&_block.data);
                        println!("{}: {}", _block.name, f32::from_be_bytes(block_data));
                    },
                    _ => { panic!("Unknown block name: {} with data {:?}", _block.name, _block.data); }
                }
            }
        println!("Packet: {:?}", apollotelemetry::construct_packet(_blocks));
        }
    }
}

