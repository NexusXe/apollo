use crate::parameters::{BARE_MESSAGE_LENGTH_BYTES, BLOCK_DELIMITER, END_HEADER_DATA, START_HEADER_DATA, BARE_MESSAGE_LENGTH_BLOCKS, TOTAL_MESSAGE_LENGTH_BYTES, FEC_EXTRA_BYTES};

extern crate reed_solomon;
use reed_solomon::Encoder;

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
pub struct Block {
    pub name: &'static str,
    pub label: u8,
    pub length: u8, // Length in bytes
    pub data: &'static [u8],
    pub do_transmit_label: bool,
}

pub fn construct_blocks<'a>(altitude: &'static [u8; 4], voltage: &'static [u8; 4], temperature: &'static [u8; 4], latitude: &'static [u8; 4], longitude: &'static [u8; 4]) -> [Block; BARE_MESSAGE_LENGTH_BLOCKS] {

    let start_header_block = Block {
        name: "Start Header",
        label: 128,
        length: START_HEADER_DATA.len() as u8,
        data: START_HEADER_DATA.as_ref(),
        do_transmit_label: false,
    };
    let altitude_block = Block {
        name: "Altitude",
        label: 129,
        length: 4,
        data: altitude,
        do_transmit_label: true,
    };
    let battery_voltage_block = Block {
        name: "Battery Voltage",
        label: 130,
        length: 4,
        data: voltage,
        do_transmit_label: true,
    };
    let temperature_block = Block {
        name: "Temperature",
        label: 131,
        length: 4,
        data: temperature,
        do_transmit_label: true,
    };
    let latitude_block = Block {
        name: "Latitude",
        label: 132,
        length: 4,
        data: latitude,
        do_transmit_label: true,
    };
    let longitude_block = Block {
        name: "Longitude",
        label: 133,
        length: 4,
        data: longitude,
        do_transmit_label: true,
    };
    let end_header_block = Block {
        name: "End Header",
        label: 255,
        length: END_HEADER_DATA.len() as u8,
        data: END_HEADER_DATA.as_ref(),
        do_transmit_label: true,
    };
    return [start_header_block, altitude_block, battery_voltage_block, temperature_block, latitude_block, longitude_block, end_header_block];
}

pub fn construct_packet(blocks: [Block; BARE_MESSAGE_LENGTH_BLOCKS]) -> [u8; BARE_MESSAGE_LENGTH_BYTES as usize] {
    // Constructs a packet from the given blocks. Each block begins with its 1 byte label attribute (if do_transmit_label is true), followed by the data. Blocks are delimited by BLOCK_DELIMITER.
    let mut packet: [u8; BARE_MESSAGE_LENGTH_BYTES as usize] = [0; BARE_MESSAGE_LENGTH_BYTES as usize];
    let mut packet_index: usize = 0;
    
    for block in blocks.iter() {
        if block.do_transmit_label {
            packet[packet_index] = block.label.to_be();
            packet_index += 1;
        }
        packet[packet_index..packet_index + block.length as usize].copy_from_slice(&block.data);
        packet_index += block.length as usize;
        
        packet[packet_index] = BLOCK_DELIMITER.to_be_bytes()[0];
        packet[packet_index + 1] = BLOCK_DELIMITER.to_be_bytes()[1];
        packet_index += 2;

    }

    return packet;
}

pub fn encode_packet(&_bare_packet: &[u8; BARE_MESSAGE_LENGTH_BYTES]) -> [u8; TOTAL_MESSAGE_LENGTH_BYTES] {
    // Encodes the given packet using the reed_solomon crate. Returns the encoded packet.
    let enc = Encoder::new(FEC_EXTRA_BYTES);
    let _encoded_packet = enc.encode(&_bare_packet[..]);
    return _encoded_packet[..].try_into().unwrap();
}
