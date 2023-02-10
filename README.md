# apollo
High-altitude amateur radio balloon.

## Packet overview:
Radio is 422MHz 4-GFSK. Text is encoded in UTF-8. Lowest to highest frequency, the bit order is `00 01 10 11`.
Messages have a start header of `0xF0F0`, followed by my callsign, KD9TFA.
