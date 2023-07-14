pub const NOMINAL_VOLTAGE: f32 = 3.6; // nominal battery voltage, in volts
pub const OLC_PRECISION: usize = 8; // number of significant digits in the Open Location Code
pub const OLC_CODE_LENGTH: usize = OLC_PRECISION + 9; // length of the Open Location Code, in characters
pub const CALLSIGN: &str = "KD9TFA"; // callsign of the balloon. MUST be an even number of characters, Space padding at the end is OK.


// packet related constants
pub const BLOCK_LENGTH: u8 = 1; // Packet length = 2^BLOCK_LENGTH bytes
pub const BLOCK_DELIMITER: u16 = 0xF0F0; // Delimiter between blocks
pub const TOTAL_MESSAGE_LENGTH: u8 = 80; // Total message length, in bytes
pub const FEC_DATA_LENGTH: u8 = TOTAL_MESSAGE_LENGTH >> BLOCK_LENGTH; // Ensures that each packet is 2^BLOCK_LENGTH bytes
pub const FEC_PACKETS: u8 = 10; // Number of FEC packets to generate
pub const FEC_TOTAL_LENGTH: u8 = FEC_DATA_LENGTH + FEC_PACKETS; // Total packets generated. Must be >= FEC_DATA_LENGTH
pub const START_HEADER: u16 = 0x1BE4; // Start of message header