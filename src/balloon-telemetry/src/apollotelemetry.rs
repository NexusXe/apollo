use crate::parameters::{OLC_CODE_LENGTH, TOTAL_MESSAGE_LENGTH, BLOCK_DELIMITER, FEC_DATA_LENGTH, FEC_TOTAL_LENGTH, BLOCK_LENGTH, END_HEADER_DATA, START_HEADER_DATA};
extern crate zfec_rs;
extern crate alloc;


#[derive(Debug)]
pub struct Block {
    pub name: &'static str,
    pub label: u8,
    pub length: u8, // Length in bytes
    pub data: &'static [u8],
    pub do_transmit_label: bool,
}

pub fn construct_blocks<'a>(olc_code: &'static [u8; OLC_CODE_LENGTH], altitude: &'static [u8; 4], voltage: &'static [u8; 4], temperature: &'static [u8; 4], latitude: &'static [u8; 4], longitude: &'static [u8; 4]) -> [Block; 8] {

    let start_header_block = Block {
        name: "Start Header",
        label: 128,
        length: START_HEADER_DATA.len() as u8,
        data: START_HEADER_DATA.as_ref(),
        do_transmit_label: false,
    };
    let olc_code_block = Block {
        name: "OLC Code",
        label: 129,
        length: OLC_CODE_LENGTH as u8,
        data: olc_code,
        do_transmit_label: true,
    };
    let altitude_block = Block {
        name: "Altitude",
        label: 130,
        length: 4,
        data: altitude,
        do_transmit_label: true,
    };
    let battery_voltage_block = Block {
        name: "Battery Voltage",
        length: 4,
        label: 131,
        data: voltage,
        do_transmit_label: true,
    };
    let temperature_block = Block {
        name: "Temperature",
        label: 132,
        length: 4,
        data: temperature,
        do_transmit_label: true,
    };
    let latitude_block = Block {
        name: "Latitude",
        label: 133,
        length: 4,
        data: latitude,
        do_transmit_label: true,
    };
    let longitude_block = Block {
        name: "Longitude",
        label: 134,
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
    return [start_header_block, olc_code_block, altitude_block, battery_voltage_block, temperature_block, latitude_block, longitude_block, end_header_block];
}

pub fn construct_packet(blocks: [Block; 8]) -> [u8; TOTAL_MESSAGE_LENGTH as usize] {
    // Constructs a packet from the given blocks. Each block begins with its 1 byte label attribute (if do_transmit_label is true), followed by the data. Blocks are delimited by BLOCK_DELIMITER.
    let mut packet: [u8; TOTAL_MESSAGE_LENGTH as usize] = [0; TOTAL_MESSAGE_LENGTH as usize];
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

pub fn encode_packet(packet: &[u8]) -> [u8; FEC_TOTAL_LENGTH * 2 as usize] {
    let encoder = zfec_rs::Fec::new(FEC_DATA_LENGTH * 2 as usize, FEC_TOTAL_LENGTH * 2 as usize).unwrap();
    let encoded_packet: [u8; TOTAL_MESSAGE_LENGTH as usize] = [0; TOTAL_MESSAGE_LENGTH as usize];
    let mut encoded_packet_index: usize = 0;
    // Each chunk is 2^BLOCK_LENGTH bits long. The message (without FEC data) is FEC_DATA_LENGTH bytes long, and FEC_TOTAL_LENGTH chunks are returned, which is the total length of the message.
    let (mut encoded_chunks, padding) = encoder.encode(&packet).unwrap();
    
    let mut encoded_message: [u8; FEC_TOTAL_LENGTH * 2 as usize] = [0; FEC_TOTAL_LENGTH * 2 as usize];
    // Copy the encoded chunks out of the vector and into the encoded message.
    for chunk in encoded_chunks.iter_mut() {
        encoded_message[encoded_packet_index..encoded_packet_index + BLOCK_LENGTH].copy_from_slice(&chunk.data);
        encoded_packet_index += BLOCK_LENGTH;
    }
    return encoded_message;
}