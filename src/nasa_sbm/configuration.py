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

class SimulationConfiguration:

    # Takes a .yaml file with simulation configurations
    def __init__(self, filePath: str):
        try:
            with open(filePath, 'r') as stream:
                data_loaded = yaml.safe_load(stream)
                self._minimalCharacteristicLength = float(
                    data_loaded['minimalCharacteristicLength'])
                self._simulationType = SimulationType(data_loaded['simulationType'].upper())
                self._sat_type = SatType(data_loaded['satType'].upper())
                self._mass_conservation = bool(data_loaded['massConservation'])
                stream.close()
        except Exception as e:
            print(f"Exception: {e}")

    @property
    def minimalCharacteristicLength(self) -> float:
        return self._minimalCharacteristicLength

    @property
    def simulationType(self) -> SimulationType:
        return self._simulationType

    @property
    def sat_type(self) -> SatType:
        return self._sat_type

    @property
    def mass_conservation(self) -> bool:
        return self._mass_conservation
