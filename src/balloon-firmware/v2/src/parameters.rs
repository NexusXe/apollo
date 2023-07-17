pub const NOMINAL_VOLTAGE: f32 = 3.6; // nominal battery voltage, in volts
pub const OLC_PRECISION: usize = 8; // number of significant digits in the Open Location Code
pub const OLC_CODE_LENGTH: usize = OLC_PRECISION + 8; // length of the Open Location Code, in characters
pub const CALLSIGN: &[u8; 6] = b"KD9TFA"; // callsign of the balloon. MUST be an even number of characters, Space padding at the end is OK.
pub const FLOAT_PRECISION: usize = 8; // number of significant digits in the floating point data

// packet related constants
pub const BLOCK_LENGTH: usize = 1; // Packet length = 2^BLOCK_LENGTH bytes
pub const BLOCK_DELIMITER: u16 = 0xF0F0; // Delimiter between blocks
pub const BARE_MESSAGE_LENGTH_BYTES: usize = 56; // Total message length, in bytes. TODO: make this dynamic
pub const BARE_MESSAGE_LENGTH_BLOCKS: usize = BARE_MESSAGE_LENGTH_BYTES >>  (2 ^ BLOCK_LENGTH); // Total message length, in blocks
pub const PACKET_LENGTH_BYTES: usize = usize::pow(2, BLOCK_LENGTH as u32); // Packet length, in bytes

pub const FEC_EXTRA_PACKETS: usize = 2; // Number of extra packets to send for FEC
pub const FEC_K: usize = BARE_MESSAGE_LENGTH_BYTES >> BLOCK_LENGTH; // Ensures that each packet is 2^BLOCK_LENGTH bytes
pub const FEC_M: usize = FEC_K + FEC_EXTRA_PACKETS;

// K is blocks in, M is blocks out. Also, only K blocks are needed to reconstruct the message.

pub const FEC_EXTRA_BYTES: usize = FEC_EXTRA_PACKETS * PACKET_LENGTH_BYTES; // Number of extra bytes to send for FEC
pub const TOTAL_MESSAGE_LENGTH_BYTES: usize = BARE_MESSAGE_LENGTH_BYTES + FEC_EXTRA_BYTES; // Total message length, in bytes

pub const START_END_HEADER: u16 = 0x1BE4; // Start of message header

pub const START_HEADER_DATA: [u8; CALLSIGN.len() + 2] = [START_END_HEADER.to_be_bytes()[0], START_END_HEADER.to_be_bytes()[1], CALLSIGN[0], CALLSIGN[1], CALLSIGN[2], CALLSIGN[3], CALLSIGN[4], CALLSIGN[5]]; // Start of message header data
pub const END_HEADER_DATA:   [u8; CALLSIGN.len() + 2] = [CALLSIGN[0], CALLSIGN[1], CALLSIGN[2], CALLSIGN[3], CALLSIGN[4], CALLSIGN[5], START_END_HEADER.to_be_bytes()[0], START_END_HEADER.to_be_bytes()[1]]; // End of message header data
