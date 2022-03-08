import yaml
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

    # Takes a .yaml file with simulation configurations
    def __init__(self, filePath):
        try:
            with open(filePath, 'r') as stream:
                data_loaded = yaml.safe_load(stream)
                self._minimalCharacteristicLength = float(
                    data_loaded['minimalCharacteristicLength'])
                self._simulationType = SimulationType(data_loaded['simulationType'].upper())
                self._sat_type = SatType(data_loaded['satType'].upper())
                stream.close()
        except Exception as e:
            print(f"Exception: {e}")

    @property
    def minimalCharacteristicLength(self):
        return self._minimalCharacteristicLength

    @property
    def simulationType(self):
        return self._simulationType

    @property
    def sat_type(self):
        return self._sat_type
