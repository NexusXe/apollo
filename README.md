Apollo is a free and open-source, homebrew, homebuilt, and highly adaptable amateur radio balloon.

All code, project files, schematics, as well as files used for this website can be found at [github.com/nexusxe/apollo](https://github.com/NexusXe/apollo)

---

## Packet overview
The primary radio transmissions are 422.1 MHz 4-GFSK. Text blocks are encoded in ASCII. Lowest to highest frequency, the bit order is `00 01 10 11`.
Messages have a start and end header of `00 01 10 11 11 10 01 00` (0x1B E4), followed by a callsign, in my case KD9TFA. This header appears as a wave on the radio waterfall, making the start and end of a transmission noticable.

Messages are constructed as a string of blocks, transmitted sequentially defined by their [label](https://apollo.nexusxe.com/posts/apollo_overview/#labels) Note that the Start Header and (if used) FEC must be the first blocks in the body, and End Header must be the last.
=Blocks are transmitted sequentially, separated by the block delimiter `0x F0 F0` followed by their label.
Since ASCII doesn't use the the first bit, all 1-byte data label sequences are greater than 127.
### Labels
| **Name**                                                                                       | **Label** | **Decimal** | **Block Data Type**      | **Block Length _(not incl. label)_** |
|------------------------------------------------------------------------------------------------|-----------|-------------|--------------------------|--------------------------------------|
| [Start Header](https://apollo.nexusxe.com/posts/apollo_packets#startend-headers/#end-header)\* | `0x80`    | 128         | `0x1B E4` + Text (ASCII) | 6+ bytes                             |
| [FEC](https://apollo.nexusxe.com/posts/apollo_FEC)                                             | `0x80`    | 129         | Hex                      | 8 bytes                              |
| [Location](https://apollo.nexusxe.com/posts/apollo_location/#open-location-code)               | `0x80`    | 130         | Text (ASCII)             | 14 bytes                             |
| [Altitude](https://apollo.nexusxe.com/posts/apollo_location/#altitude)                         | `0x81`    | 131         | Float (Meters)           | 4 bytes                              |
| [Battery Voltage](https://apollo.nexusxe.com/posts/apollo_sensors/#battery)                    | `0x82`    | 132         | Float (delta nom. volts) | 4 bytes                              |
| [Temperature](https://apollo.nexusxe.com/posts/apollo_sensors/#temperature)                    | `0x83`    | 133         | Float (deg. C)           | 4 bytes                              |
| [Latitude](https://apollo.nexusxe.com/posts/apollo_location/#gps)                              | `0x84`    | 134         | Float                    | 4 bytes                              |
| [Longitude](https://apollo.nexusxe.com/posts/apollo_location/#gps)                             | `0x85`    | 135         | Float                    | 4 bytes                              |
| [End Header](https://apollo.nexusxe.com/posts/apollo_packets#startend-headers/#end-header)\*   | `0xFF`    | 255         | `0x1B E4` + Text (ASCII) | 6 bytes                              |

\*<sub>Note: By default, the Start Header omits transmitting its label for the sake of identifying faster. However, the End Header (defined as the same type) doesn't do this, based on its Label. This behavior can be changed. </sub>

## Hardware overview

Microcontroller or something
