
import numpy as np
from nasabreakup.configuration import SatType, SimulationConfiguration
from nasabreakup.model import BreakupModel
from nasabreakup.satellite import Satellite

class MySat(Satellite):

    @property
    def position(self) -> np.ndarray:
        return np.array([1000, 2000, 3000])

    @property
    def velocity(self) -> np.ndarray:
        return np.array([100, 200, 300])

    @property
    def mass(self) -> float:
        return 500

    @property
    def characteristic_length(self) -> float:
        return ((6.0 * self.mass) / (92.937 * np.pi)) ** (1.0 / 2.26)

    @property
    def type(self) -> SatType:
        return SatType.sat

sat = MySat()
config = SimulationConfiguration('sample_config.yaml')
event  = BreakupModel(config, np.array([sat]))

debris = event.run()