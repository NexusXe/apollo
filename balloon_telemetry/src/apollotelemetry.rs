use crate::parameters::{CALLSIGN, OLC_CODE_LENGTH, TOTAL_MESSAGE_LENGTH, BLOCK_DELIMITER};

#[derive(Debug)]
pub struct Block {
    pub name: &'static str,
    pub label: u16,
    pub length: u8, // Length in bytes
    pub data: &'static [u8],
    pub do_transmit_label: bool,
}

pub fn construct_blocks<'a>(olc_code: &'static [u8; 16], altitude: &'static [u8; 4], voltage: &'static [u8; 4], temperature: &'static [u8; 4], latitude: &'static [u8; 4], longitude: &'static [u8; 4]) -> [Block; 8] {

    let start_header_block = Block {
        name: "Start Header",
        label: 128,
        length: 6,
        data: CALLSIGN.as_bytes(),
        do_transmit_label: false,
    };
    let olc_code_block = Block {
        name: "OLC Code",
        label: 129,
        length: OLC_CODE_LENGTH as u8 - 1u8,
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
        length: 6,
        data: CALLSIGN.as_bytes(),
        do_transmit_label: true,
    };
    return [start_header_block, olc_code_block, altitude_block, battery_voltage_block, temperature_block, latitude_block, longitude_block, end_header_block];
}

pub fn construct_packet(blocks: [Block; 8]) -> [u8; TOTAL_MESSAGE_LENGTH as usize] {
    // Constructs a packet from the given blocks. Each block begins with a 2-byte label (if do_transmit_label is true), followed by the data. Blocks are delimited by BLOCK_DELIMITER.
    let mut packet: [u8; TOTAL_MESSAGE_LENGTH as usize] = [0; TOTAL_MESSAGE_LENGTH as usize];
    let mut packet_index: usize = 0;
    for block in blocks.iter() {
        if block.do_transmit_label {
            packet[packet_index] = (block.label >> 8) as u8;
            packet_index += 1;
            packet[packet_index] = block.label as u8;
            packet_index += 1;
        }
        packet[packet_index..packet_index + block.length as usize].copy_from_slice(block.data);
        packet_index += block.length as usize;
        packet[packet_index] = (BLOCK_DELIMITER >> 8) as u8;
        packet_index += 1;
        packet[packet_index] = BLOCK_DELIMITER as u8;
        packet_index += 1;
    }
    return packet;
}