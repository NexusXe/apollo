#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
#![feature(core_intrinsics)]

extern crate micromath;

extern crate libm;


use arduino_hal::prelude::*;
use panic_halt as _;

pub mod parameters;
pub mod apollosensors;
pub mod apollotelemetry;


pub static mut _latitude: [u8; 4] = [0u8; 4];
pub static mut _longitude: [u8; 4] = [0u8; 4];
pub static mut _altitude: [u8; 4] = [0u8; 4];
pub static mut _voltage: [u8; 4] = [0u8; 4];
pub static mut _temperature: [u8; 4] = [0u8; 4];
pub static mut _blocks: [apollotelemetry::Block; 7] = [apollotelemetry::Block {
    name: "Start Header",
    label: 0,
    length: 0,
    data: &[0u8; 0],
    do_transmit_label: false,
}; 7];


pub fn generate_packet() -> [u8; parameters::TOTAL_MESSAGE_LENGTH_BYTES] {
    unsafe { // TODO: horrific
        

        (_latitude, _longitude) = apollosensors::get_location();
        _altitude = apollosensors::get_altitude();
        _voltage = apollosensors::get_voltage();
        _temperature = apollosensors::get_temperature();

        // quick testing values

        _latitude = 69.1337f32.to_be_bytes();
        _longitude = 69.420f32.to_be_bytes();
        _altitude = 1337.69f32.to_be_bytes();
        _voltage = 420.69f32.to_be_bytes();
        _temperature = 420.1337f32.to_be_bytes();

        _blocks = apollotelemetry::construct_blocks(&_altitude, &_voltage, &_temperature, &_latitude, &_longitude);
    

        for _block in _blocks.iter() {
            assert_eq!(_block.length, _block.data.len() as u8);
        }

        let _packet = apollotelemetry::construct_packet(_blocks);

        let _encoded_packet = apollotelemetry::encode_packet(&_packet);


        return _encoded_packet;
    }
    
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    // Set clock speed to 24 MHz


    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);

    let mut i: u32 = 0;

    loop {
        
        for x in 0..1000 {
            let _packet = generate_packet();
        }
        i += 1;
        ufmt::uwriteln!(&mut serial, "Packet count: {}000", i).void_unwrap();
        
        arduino_hal::delay_us(100);
        // Print each block name and data in the packet
        unsafe {
            for &_block in _blocks.iter() {
                match _block.name {
                    "Start Header" | "End Header" => {
                        ufmt::uwriteln!(&mut serial, "{} {:?}", _block.name, _block.data).void_unwrap();
                    }, // for the start and end header blocks, print the data as a byte array. for all other blocks, print the data as a formatted float.
                    // because we don't have an allocator, we can't use the uDebug trait
                    // in addition, since f32 doesn't implement uDisplay, we can't use ufmt::uwriteln! to format the data.
                    // we have to format it manually, without the format! macro, alloc, or String.
                    _ => { // TODO: this sucks, is slow, is imprecise, and uses 4.8K of flash. fix it.
                        // format the f32 data, with 8 decimal places
                        let _data: f32 = f32::from_be_bytes(_block.data.try_into().unwrap());
                        let (_data, _data_decimal) = format_f32(_data);
                        ufmt::uwriteln!(&mut serial, "{} {}.{}", _block.name, _data, _data_decimal).void_unwrap();
                }
            }
            // generate test CRC, which is the inverse square root of the sum of all bytes in the data fields of all blocks
            let mut _crc_sum: u128 = 0;
            for _block in _blocks.iter() {
                for _byte in _block.data.iter() {
                    _crc_sum += *_byte as u128;
                }
            
            let _crc_sum = _crc_sum as f32;

            
            // this method of doing CRC is haunted.
            //let _crc_sum = 1.0 / micromath::F32Ext::sqrt(_crc_sum);
            //let _crc_sum = 1.0 / _crc_sum.sqrt();
            
            //let (_crc_sum, _crc_sum_decimal) = format_f32(_crc_sum);
            //ufmt::uwriteln!(&mut serial, "CRC {}.{}", _crc_sum, _crc_sum_decimal).void_unwrap();

            }

        }

        }
    }
}

fn format_f32(_data: f32) -> (u32, u32) {
    // because we don't have an allocator, we can't use the uDebug trait
    // in addition, since f32 doesn't implement uDisplay, z! macro, alloc, or String.
    // TODO: this sucks, is slow, is imprecise, and uses 4.8K of flash. fix it.
    let _data = _data;
    let _data_whole_number: f32 = libm::floorf(_data);
    let _data_decimal: f32 = _data - _data_whole_number;

    let _data: u32 = (_data * 10000.0) as u32;
    return (_data / 10000, _data % 10000); // 32 bit integer division is slow, but at least it isn't 32 bit float division
}