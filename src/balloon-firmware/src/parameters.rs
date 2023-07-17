pub const NOMINAL_VOLTAGE: f32 = 3.6; // nominal battery voltage, in volts
pub const OLC_PRECISION: usize = 8; // number of significant digits in the Open Location Code
pub const OLC_CODE_LENGTH: usize = OLC_PRECISION + 8; // length of the Open Location Code, in characters
pub const CALLSIGN: &[u8; 6] = b"KD9TFA"; // callsign of the balloon. MUST be an even number of characters, Space padding at the end is OK.


// packet related constants
pub const BLOCK_LENGTH: usize = 1; // Packet length = 2^BLOCK_LENGTH bytes
pub const BLOCK_DELIMITER: u16 = 0xF0F0; // Delimiter between blocks
pub const TOTAL_MESSAGE_LENGTH: usize = 80; // Total message length, in bytes
pub const FEC_DATA_LENGTH: usize = TOTAL_MESSAGE_LENGTH >> BLOCK_LENGTH; // Ensures that each packet is 2^BLOCK_LENGTH bytes
pub const FEC_PACKETS: usize = 10; // Number of FEC packets to generate
pub const FEC_TOTAL_LENGTH: usize = FEC_DATA_LENGTH + FEC_PACKETS; // Total packets generated. Must be >= FEC_DATA_LENGTH

pub const START_END_HEADER: u16 = 0x1BE4; // Start of message header

pub const START_HEADER_DATA: [u8; CALLSIGN.len() + 2] = [START_END_HEADER.to_be_bytes()[0], START_END_HEADER.to_be_bytes()[1], CALLSIGN[0], CALLSIGN[1], CALLSIGN[2], CALLSIGN[3], CALLSIGN[4], CALLSIGN[5]]; // Start of message header data
pub const END_HEADER_DATA:   [u8; CALLSIGN.len() + 2] = [CALLSIGN[0], CALLSIGN[1], CALLSIGN[2], CALLSIGN[3], CALLSIGN[4], CALLSIGN[5], START_END_HEADER.to_be_bytes()[0], START_END_HEADER.to_be_bytes()[1]]; // End of message header data