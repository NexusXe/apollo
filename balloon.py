from typing import Tuple

from numpy import float32
import apollotelemetry as at
from zfec import easyfec
from parameters import FEC_DATA_LENGTH, FEC_TOTAL_LENGTH, BLOCK_LENGTH, FEC_PACKETS, BLOCK_DELIMITER
import random
import numpy as np

HEADER_CALLSIGN: str = "KD9TFA"
blocks: list[at.Block] = at.construct_blocks()

BLOCK_NAMES: tuple[str] = tuple([block.name for block in blocks])
BLOCK_LABELS: tuple[int] = tuple([block.label for block in blocks])
BLOCK_TYPES: tuple[type | list[type, type] | None] = tuple([block.datatype for block in blocks])
BLOCK_LENGTHS: tuple[int] = tuple([block.length for block in blocks])
BLOCK_DO_TRANSMIT_LABELS: tuple[bool] = tuple([block.do_transmit_label for block in blocks])


def decode_packet(packet: tuple[bytes], positions: tuple[int]) -> bytes:
    """
    Decodes a packet, using its FEC to reclaim any lost data
    :param packet: The packet to decode
    :type: packet: list[bytes]
    :param positions: The number location of the packets
    :type: positions: tuple[int]
    :return: The decoded packet
    :rtype: bytes
    """
    return easyfec.Decoder(FEC_DATA_LENGTH, FEC_TOTAL_LENGTH - 1).decode(packet, positions, 0)


def generate_packet(_blocks: list[at.Block], olc_code: str,
                    altitude: np.float32, voltage: np.float32,
                    temp: np.float32, instant_lat: np.float32,
                    instant_long: np.float32) -> tuple[bytearray]:
    """
    Generates a packet, using the given parameters
    :param _blocks: Constructed blocks
    :type: _blocks: list[Block]
    :param olc_code: Open Location Code
    :type: olc_code: str
    :param altitude: Current normalised altitude above sea level, in meters
    :type: altitude: np.float32
    :param voltage: Current normalised voltage deviation from nominal, in volts
    :type: voltage: np.float32
    :param temp: Current normalised temperature, in degrees Celsius
    :type: temp: np.float32
    :param instant_lat: Instantaneous latitude, in degrees
    :type: instant_lat: np.float32
    :param instant_long: Instantaneous longitude, in degrees
    :type: instant_long: np.float32
    :return: The generated packet
    :rtype: tuple[bytearray]
    """

    location: tuple[str, float32, float32] = at.get_location()  # GPS call
    at.populate_blocks(_blocks, HEADER_CALLSIGN, olc_code, altitude, voltage, temp, instant_lat, instant_long)
    return at.build_packet(_blocks)


def chunk_packet(_packet: bytearray) -> tuple[bytearray]:
    """
    Chunks a packet into FEC_DATA_LENGTH chunks
    :param _packet: The packet to chunk
    :type _packet: bytearray
    :return: The chunked packet
    :rtype: tuple[bytearray]
    """
    return tuple(bytearray(_packet[i:i + 2]) for i in range(0, len(_packet), 2))


def transmit_and_receive(_packet: bytearray) -> tuple[bytearray]:
    """
    Simulates transmission and reception of a packet by corrupting some bytes
    :param _packet: The packet to corrupt
    :type _packet: list[bytearray]
    :return: The packet with corruption that is tolerable by FEC
    :rtype: tuple[bytearray]
    """

    _packet: list[bytearray] = list(chunk_packet(_packet))
    # Fill FEC_DATA_LENGTH blocks with 2 ** BLOCK_LENGTH random bytes
    for i in range(FEC_DATA_LENGTH):
        _packet[np.random.randint(0, FEC_DATA_LENGTH + 1)] = bytearray(np.random.bytes(2 ** BLOCK_LENGTH))
    return tuple(_packet)


def main() -> None:
    if True:
        packet: tuple[bytearray] = generate_packet(blocks, at.get_location()[0], at.get_altitude(), at.get_voltage(),
                                                   at.get_temperature(), at.get_location()[1], at.get_location()[2])
        # print(packet)
        received_packet: bytearray = at.generate_fec(packet)
        # print(received_packet)
        structured_packet: tuple[bytes] = chunk_packet(received_packet)

        assert len(tuple(range(structured_packet.__len__()))) == len(structured_packet)
        nums_and_blocks: tuple[tuple[int, bytes], ...] = tuple(zip(range(len(structured_packet)), structured_packet,
                                                                   strict=True))

        nums_and_blocks = random.sample(nums_and_blocks, FEC_DATA_LENGTH)
        nums_and_blocks.sort(key=lambda x: x[0])
        # print(nums_and_blocks)
        decoded_packet = decode_packet(tuple([x[1] for x in nums_and_blocks]),
                                       tuple([x[0] for x in nums_and_blocks])).hex()
        # print(received_packet.hex())
        # print(decode_packet(structured_packet, tuple(range(structured_packet.__len__()))))
        corrupted_packet = transmit_and_receive(bytearray(b"".join(structured_packet)))
        mended_packet = repair_packet(corrupted_packet)
        print(mended_packet)


def repair_packet(_packet: tuple[bytearray]) -> tuple[bytearray]:
    # Before FEC, we can recreate the packet in its entirety (minus block data and FEC) to increase our chances of
    # recovering the data.

    known_blocks: list[at.Block] = [at.Block(BLOCK_NAMES[i], BLOCK_LABELS[i], BLOCK_LENGTHS[i],
                                             BLOCK_TYPES[i], do_transmit_label=BLOCK_DO_TRANSMIT_LABELS[i])
                                    for i in range(0, len(BLOCK_NAMES))]

    packet_skeleton = generate_packet(known_blocks, olc_code="00000000+0000000", altitude=np.float32(0),
                                      voltage=np.float32(0), temp=np.float32(0), instant_lat=np.float32(0),
                                      instant_long=np.float32(0))
    print(f"Packet Skeleton: {packet_skeleton}")
    _packet = b"".join(_packet)
    print(_packet)
    assert len(_packet) == len(b"".join(packet_skeleton)) + (FEC_PACKETS * (2 ** BLOCK_LENGTH)), \
        f"Packet length mismatch:\nCorrupted packet - {len(_packet)} -- {_packet}" \
        f"\nPacket Skeleton  - {len(b''.join(packet_skeleton)) + (FEC_PACKETS * (2 ** BLOCK_LENGTH))} -- " \
        f"{b''.join(packet_skeleton)} plus {FEC_PACKETS * (2 ** BLOCK_LENGTH)} bytes of FEC"

    packet_skeleton_delimiters = []

    i = 0
    a = 1
    for x in packet_skeleton[i:]:  # @TODO: This is a bit of a mess, clean it up, but works for now
        if BLOCK_DELIMITER.to_bytes(2 ** BLOCK_LENGTH, "big") in packet_skeleton[i:]:
            a: int = packet_skeleton[i:].index(BLOCK_DELIMITER.to_bytes(2 ** BLOCK_LENGTH, "big"))
            if packet_skeleton_delimiters:
                packet_skeleton_delimiters.append(a + packet_skeleton_delimiters[-1] if packet_skeleton_delimiters[-1]
                                                                                        == 2 else
                                                  packet_skeleton_delimiters[
                                                      -1] + 3)  # Might need to be changed if we change the number of
                # packets between delimiters
            else:
                packet_skeleton_delimiters.append(a)
            i += (a + 1)

    print(packet_skeleton_delimiters)

    for i in packet_skeleton_delimiters:
        print(packet_skeleton[i])

    # We can now use the packet skeleton to determine which blocks are missing, and then use FEC to recover them


def packet_parser(_packet: tuple[bytearray]) -> tuple[tuple[str, int, type, bytearray]]:
    """
    Parses a packet into a tuple of tuples, with each tuple containing the name, label, type, and data of that block
    :param _packet: Received packet
    :type _packet: tuple[bytes]
    :return: The parsed packet
    :rtype: tuple[tuple[str, int, type, bytes]]
    """
    assert len(BLOCK_NAMES) == len(BLOCK_LABELS) == len(BLOCK_TYPES) == len(BLOCK_LENGTHS) == \
           len(BLOCK_DO_TRANSMIT_LABELS)

    def parse_block(block_name: str, block_label: int, block_type: type | tuple[type, type],
                    block_data: bytes) -> tuple[str, int, type, bytes]:
        pass

    print(repair_packet(_packet))


if __name__ == "__main__":
    main()
