import builtins
import numpy as np
from zfec import easyfec
from apollosensors import *
from parameters import FEC_DATA_LENGTH, FEC_TOTAL_LENGTH, CALLSIGN, BLOCK_DELIMITER

# declare constants


def convert_to_bytes(input_data: str | int | np.float32 | bytes | bytearray | tuple | list) -> bytes:
    match type(input_data):
        case builtins.str:
            return bytes(input_data, "ascii")
        case builtins.int:
            return bytes(input_data.to_bytes(2, "big"))
        case np.float32:
            return bytes(input_data.tobytes())
        case builtins.bytes:
            return input_data
        case builtins.bytearray:
            return bytes(input_data)
        case builtins.list:
            chunks_together = bytearray()  # initialize empty bytearray for concatenation
            for subchunk in input_data:
                subchunk_bytes = convert_to_bytes(subchunk)
                chunks_together += subchunk_bytes
            return bytes(chunks_together)
        case _:
            raise TypeError(f"Unknown type in packet skeleton, {type(input_data)}, {input_data}")


class Block:
    """
    A class to represent a block of data in the message
    """

    def __str__(self):
        return f"block: {self.name}, label: {self.label}, length: {self.length}, data type: {self.datatype}," \
               f"data: {self.data}, do_transmit_label: {self.do_transmit_label}"

    def __repr__(self):
        return f"apollotelemetry.Block({self.name}, {self.label}, {self.length}, {self.datatype}, {self.data}," \
               f"{self.do_transmit_label})"

    def __init__(self, name: str, label: int, length: int, datatype: type | list[type, type] = None, data=None,
                 do_transmit_label: bool = True):
        """
        Parameters:
        :param name: Name of the block, for reference
        :type: name: str
        :param label: Label of the block; determines the position of the block in the message
        :type: label: int
        :param length: Length of the block, in bytes
        :type: length: int
        :param datatype: Data type(s) of the block
        :type: datatype: type | list[type, type]
        :param data: Data of the block
        :type: data: type(datatype)
        :param do_transmit_label: Whether to transmit the label of the block (for the Start Header)
        :type: do_transmit_label: bool
        """
        self.label = label
        self.name = name
        self.length = length
        self.datatype = datatype
        if datatype is np.float32 and type(data) is int:
            self.data: np.float32 = np.float32(data)
        else:
            self.data: type(datatype) | None = data
        self.do_transmit_label = do_transmit_label

        if type(datatype) is list:
            assert type(data) is list, "Data must be a list if the data type is a list"
            # Turn the mixed data into a single hexadecimal string


def construct_blocks() -> list[Block]:
    """
    Constructs the blocks of the message
    :return: list[Block]
    """
    # Block Definitions:
    # | **Name**        | **Label** | **Decimal** | **Block Data Type**      | **Block Length _(not incl. label)_** |
    # |-----------------|-----------|-------------|--------------------------|--------------------------------------|
    # | Start Header    | `0x80`    | 128         | `0x1B E4` + Text (ASCII) | 8*n bytes                            |
    # | Location        | `0x81`    | 129         | Text (ASCII)             | 16 bytes                             |
    # | Altitude        | `0x82`    | 130         | Float32 (Meters)         | 4 bytes                              |
    # | Battery Voltage | `0x83`    | 131         | Float32 (delta volts)    | 4 bytes                              |
    # | Temperature     | `0x84`    | 132         | Float32 (deg. C)         | 4 bytes                              |
    # | Latitude        | `0x85`    | 133         | Float32                  | 4 bytes                              |
    # | Longitude       | `0x86`    | 134         | Float32                  | 4 bytes                              |
    # | End Header      | `0xFF`    | 255         | Text (ASCII) + `0x1B E4` | 6 bytes                              |

    # Location is an Open Location Code with a length of 8 significant digits.
    # Altitude is in meters.
    # Battery Voltage is the difference between the nominal voltage (12 volts) and the actual voltage, in volts.
    # Temperature is in degrees Celsius.
    # Latitude and Longitude are in decimal degrees.

    # Blocks are transmitted sequentially in the packet,
    # separated by BLOCK_DELIMITER followed by their label in hexadecimal.

    blockList = []
    # Constructing the blocks, without data for now
    startHeader = Block("Start Header", 128, 6, int, do_transmit_label=False)
    location = Block("Location", 129, 14, str)
    altitude = Block("Altitude", 130, 4, np.float32)
    batteryVoltage = Block("Battery Voltage", 131, 4, np.float32)
    temperature = Block("Temperature", 132, 4, np.float32)
    latitude = Block("Latitude", 133, 4, np.float32)
    longitude = Block("Longitude", 134, 4, np.float32)
    endHeader = Block("End Header", 255, 6, int)

    # Adding the blocks to the block list
    blockList.append(startHeader)
    blockList.append(location)
    blockList.append(altitude)
    blockList.append(batteryVoltage)
    blockList.append(temperature)
    blockList.append(latitude)
    blockList.append(longitude)
    blockList.append(endHeader)
    return blockList


# Default setup of the blocks. This can be redefined by the user.
def populate_blocks(block_list: list[Block], header_callsign: str = CALLSIGN, olc_code: str = "ABC12345+67890",
                    measured_alt: np.float32 = np.float32(0.0), delta_voltage: np.float32 = np.float32(100),
                    measured_temp: np.float32 = np.float32(0.0), instant_lat: np.float32 = np.float32(0.0),
                    instant_long: np.float32 = np.float32(0.0)) -> None:
    """
    Populates the data of the blocks
    :param block_list: List of blocks, generated by construct_blocks()
    :type: block_list: list[Block]
    :param header_callsign: Callsign of the balloon/operator
    :type: header_callsign: str
    :param olc_code: Open Location Code
    :type: olc_code: str
    :param measured_alt: Measured altitude
    :type: measured_alt: np.float32
    :param delta_voltage: Difference between the nominal voltage (12 volts) and the actual voltage, in volts
    :type: delta_voltage: np.float32
    :param measured_temp: Measured temperature, in degrees Celsius
    :type: measured_temp: np.float32
    :param instant_lat:  Instantaneous latitude
    :type: instant_lat: np.float32
    :param instant_long: Instantaneous longitude
    :type: instant_long: np.float32
    :return: None
    """

    # Populating the data of the blocks
    block_list[0].data = [0x1BE4, header_callsign]  # Start Header
    block_list[1].data = olc_code  # Location
    block_list[2].data = measured_alt  # Altitude
    block_list[3].data = delta_voltage  # Battery Voltage
    block_list[4].data = measured_temp  # Temperature
    block_list[5].data = instant_lat  # Latitude
    block_list[6].data = instant_long  # Longitude
    block_list[7].data = [header_callsign, 0x1BE4]  # End Header


def build_packet(block_list: list[Block]) -> tuple[bytearray]:  # Build the packet
    """
    Builds the packet
    :param block_list: List of blocks, generated by construct_blocks() and
    populated by populate_blocks()
    :type: block_list: list[Block]
    :return: tuple[bytes] - The packet, ready to be parsed in-order and transmitted. This preserves block structure.
    """
    packet_skeleton: list[str | int | np.float32] = []

    # Adding the blocks to the packet
    for block in block_list:
        # Add the block delimiter and the label
        if block.do_transmit_label:
            packet_skeleton.append(BLOCK_DELIMITER)
            packet_skeleton.append(block.label)
        packet_skeleton.append(block.data)

    # Turning the packet into a list of bytes
    packet_skeleton_bytes = []
    for nibble in packet_skeleton:
        packet_skeleton_bytes.append(bytearray(convert_to_bytes(nibble)))
    return tuple(packet_skeleton_bytes)


def generate_fec(packet: tuple[bytearray]) -> bytearray:
    """
    Generates the FEC code for the packet, from zfec library
    :param packet: The packet, generated by build_packet()
    :type: packet: tuple[bytearray]
    :return: bytes - The FEC code, appended to the end of the packet
    :rtype: bytearray
    """

    FECOut = easyfec.Encoder(FEC_DATA_LENGTH, FEC_TOTAL_LENGTH).encode(b"".join(packet))
    # print([bytes.hex(x) for x in FECOut])
    return bytearray(b"".join(FECOut))


def main():
    newBlockList = construct_blocks()
    currentOLC, currentLocationLat, currentLocationLong = get_location()
    populate_blocks(newBlockList, CALLSIGN, currentOLC, get_altitude(), get_voltage(), get_temperature(),
                    currentLocationLat, currentLocationLong)
    newPacket = build_packet(newBlockList)
    fecData = generate_fec(newPacket)
    # print(fecData)
    newPacket = b"".join(newPacket)
    print(f"length: {len(newPacket)} --- {newPacket}")
    print(f"length: {len(fecData)} --- {fecData}")


if __name__ == "__main__":
    main()
