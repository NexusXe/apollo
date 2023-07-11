BATTERY_VOLTAGE = 12  # nominal battery voltage, in volts
OLC_PRECISION = 8  # number of significant digits in the Open Location Code
CALLSIGN = "KD9TFA"  # callsign of the balloon. MUST be an even number of characters. Space padding at the end is OK.

# packet-related constants
BLOCK_LENGTH = 1  # Packet length = 2^BLOCK_LENGTH bytes
BLOCK_DELIMITER = 0xF0F0  # Delimiter between blocks
TOTAL_MESSAGE_LENGTH = 80  # Total message length, in bytes
FEC_DATA_LENGTH: int = TOTAL_MESSAGE_LENGTH >> BLOCK_LENGTH  # Ensures that each packet is 2^BLOCK_LENGTH bytes
FEC_PACKETS = 10  # Number of FEC packets to generate
FEC_TOTAL_LENGTH = FEC_DATA_LENGTH + FEC_PACKETS  # Total packets generated. Must be >= FEC_DATA_LENGTH

if __name__ == "__main__":
    import apollotelemetry
    apollotelemetry.main()
