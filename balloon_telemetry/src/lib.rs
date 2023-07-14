#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![no_std]

extern crate geo_types;
extern crate open_location_code;
extern crate rand;
extern crate alloc;

mod parameters;
mod apollosensors;
mod apollotelemetry;
mod private;
pub use private::generate_packet;

