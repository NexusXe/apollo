#![no_std]
#![no_main]

use libc_print::std_name::println;

use crate::parameters;
use crate::apollosensors;
use crate::apollotelemetry;


pub fn generate_packet() {
    unsafe { // TODO: horrific
        pub static mut _olc_code: [u8; parameters::OLC_CODE_LENGTH] = [0u8; parameters::OLC_CODE_LENGTH];
        pub static mut _latitude: [u8; 4] = [0u8; 4];
        pub static mut _longitude: [u8; 4] = [0u8; 4];
        pub static mut _altitude: [u8; 4] = [0u8; 4];
        pub static mut _voltage: [u8; 4] = [0u8; 4];
        pub static mut _temperature: [u8; 4] = [0u8; 4];
        pub static mut _bare_packet: [u8; parameters::TOTAL_MESSAGE_LENGTH] = [0u8; parameters::TOTAL_MESSAGE_LENGTH];
        pub static mut _working_packet: [u8; parameters::FEC_TOTAL_LENGTH * 2] = [0u8; parameters::FEC_TOTAL_LENGTH * 2];

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
        if cfg!(debug_assertions) {
            for _block in _blocks.iter() {
                match _block.name{
                    "Start Header" | "End Header" => {
                        println!("{}: {:x?}", _block.name, _block.data);
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
            // Ensure that each block is BLOCK_LENGTH bytes long.
            for _block in _blocks.iter() {
                assert_eq!(_block.length, _block.data.len() as u8);
            }
            
            _bare_packet = apollotelemetry::construct_packet(_blocks);
            println!("Bare packet: {:x?}\nBare packet length: {}", _bare_packet, _bare_packet.len());
            _working_packet = apollotelemetry::encode_packet(&_bare_packet);
            println!("Working packet: {:x?}\nWorking packet length: {}", _working_packet, _working_packet.len());
        }
    }
}

