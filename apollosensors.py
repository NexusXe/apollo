from numpy import float32
import random
import openlocationcode as olc
from parameters import OLC_PRECISION, BATTERY_VOLTAGE


# Sensor-related functions (to be replaced with actual sensor code)

def get_location() -> (str, float32, float32):
    """
    Gets the location of the balloon
    :return: (str, np.float32, np.float32) - Open Location Code, latitude, longitude
    """
    # The location is an Open Location Code with a length of OLC_PRECISION significant digits.
    try:
        # This is a placeholder function. TODO: Replace with actual GPS code.
        lat = random.uniform(-90, 90)
        long = random.uniform(-180, 180)
    except (Exception,):
        lat = 0
        long = 0
    return olc.encode(lat, long, OLC_PRECISION + 9), float32(lat), float32(long)


def get_altitude() -> float32:
    """
    Gets the altitude of the balloon
    :return: float32 - Altitude, in meters
    """
    try:
        # This is a placeholder function. TODO: Replace with actual altitude code.
        altitude = float32(random.uniform(0, 10000))
    except (Exception,):
        altitude = float32(0.0)
    return altitude


def get_voltage() -> float32:
    """
    Gets the voltage difference from nominal of the balloon
    :return: float32 - Delta volts from nominal, in volts
    """
    try:
        # This is a placeholder function. TODO: Replace with actual voltage code.
        voltage = float32(BATTERY_VOLTAGE - random.uniform(0, 12))
    except (Exception,):
        voltage = float32(100.0)
    return voltage


def get_temperature() -> float32:
    """
    Gets the temperature of the balloon
    :return: float32 - Temperature, in degrees Celsius
    """
    try:
        # This is a placeholder function. TODO: Replace with actual temperature code.
        temperature = float32(random.uniform(-50, 50))
    except (Exception,):
        temperature = float32(0.0)
    return temperature
