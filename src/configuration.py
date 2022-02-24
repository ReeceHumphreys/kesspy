import configparser
from enum import Enum


class SimulationType(Enum):
    explosion = "EXPLOSION"
    collision = "COLLISION"


class SatType(Enum):
    rb = "RB"
    sat = "SC"
    soc = "SOC"
    deb = "DEB"

    @property
    def index(self):
        if self == SatType.rb:
            return 0
        elif self == SatType.sat:
            return 1
        elif self == SatType.soc:
            return 2
        else:
            return 3


class SimulationConfiguration:

    # Takes a .ini file with simulation configurations
    def __init__(self, filePath):
        parser = configparser.ConfigParser()
        parser.read(filePath)
        self._minimalCharacteristicLength = float(
            parser.get("simulation", "minimalCharacteristicLength")
        )
        self._simulationType = SimulationType(
            parser.get("simulation", "simulationType")
        )
        self._sat_type = SatType(parser.get("simulation", "satType"))

    @property
    def minimalCharacteristicLength(self):
        return self._minimalCharacteristicLength

    @property
    def simulationType(self):
        return self._simulationType

    @property
    def sat_type(self):
        return self._sat_type
