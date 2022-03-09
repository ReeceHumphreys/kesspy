from .event import Explosion, Collision
from .configuration import SimulationType, SatType, SimulationConfiguration

from .utils import (
    mean_1,
    mean_2,
    mean_soc,
    sigma_1,
    sigma_2,
    sigma_soc,
    alpha,
    power_law
)
import numpy as np


class BreakupModel:

    _output = np.array([])

    @property
    def simulation_type(self) -> SimulationType:
        return self._simulation_type

    @simulation_type.setter
    def simulation_type(self, value):
        self._simulation_type = value
        if value == SimulationType.explosion:
            self._event = Explosion()
        else:
            self._event = Collision()

    # Sats is an array containing the satellites involved in the
    # fragmentation event Will contain one for explosions and
    # two for collisions
    def __init__(self, config : SimulationConfiguration, sats : np.ndarray):

        self.sats = sats

        # Explosion or Collision
        self.simulation_type = config.simulationType

        # Setting characteristic lengths
        self._min_characteristic_length = config.minimalCharacteristicLength

        # Setting if mass should be conserved
        self._mass_conservation_enabled = config.mass_conservation

    def run(self):
        # Compute the number of fragments generate in the fragmentation event
        count = self._event.fragment_count(
            self.sats, self._min_characteristic_length
        )
        # Location the explosion occured
        r = self.sats[0].position

        # Assigning debris type and location
        self._output = np.empty((count, 7, 3))
        self._output[:, 0] = None
        self._output[:, 1] = r

        # Computing L_c for each debris following powerlaw
        self._output[:, 2] = self._characteristic_length_distribution()
        for i in range(count):
            # Computing A/M ratio for debris
            self._output[i, 3] = self._AM_Ratio(self._output[i, 2, 0])
            # Computing Area for each debris using L_c
            self._output[i, 4] = self._compute_Area(self._output[i, 2, 0])
        # Compute Mass using area and AM ratio
        self._output[:, 5] = self._compute_mass(
            self._output[:, 4, :], self._output[:, 3, :]
        )

        # Mass conservation
        if self._mass_conservation_enabled == True:
            self._conserve_mass()

        count = self._output.shape[0]

        # Determine debris velocity
        self._output[:, 6] = self.sats[0].velocity
        for i in range(count):
            chi = np.log10(self._output[i, 3, 0])
            mean = (
                self._event.delta_velocity_offset[0] * chi
                + self._event.delta_velocity_offset[1]
            )
            sigma = 0.4
            n = np.random.normal(mean, sigma)
            velocity_scalar = pow(10.0, n)

            # Transform the scalar velocity into a vector
            ejection_velocity = self._velocity_vector(velocity_scalar)
            velocity = self._output[i, 6, :] + ejection_velocity
            self._output[i, 6] = velocity

        return self._output

    def _velocity_vector(self, velocity : float) -> np.ndarray:
        n1 = np.random.uniform(0.0, 1)
        n2 = np.random.uniform(0.0, 1)
        u = n1 * 2.0 - 1.0
        theta = n2 * 2.0 * np.pi
        v = np.sqrt(1.0 - u * u)
        return np.array(
            [
                v * np.cos(theta) * velocity,
                v * np.sin(theta) * velocity,
                u * velocity,
            ]
        )

    def _conserve_mass(self):

        # Enforce Mass Conservation if the output mass is greater than the
        # input mass
        output_mass = np.sum(self._output[:, 5, 0])
        old_length = self._output.shape[0]
        new_length = old_length

        while output_mass > self._event.input_mass:
            self._output = np.delete(self._output, -1, 0)
            output_mass = np.sum(self._output[:, 5, 0])
            new_length = self._output.shape[0]

        if old_length != new_length:
            print("TODO: Removed debris to bring output mass close to input")
        else:
            while self._event.input_mass > output_mass:
                new_row = np.empty((7, 3))
                new_row[0] = None
                new_row[1] = self.sats[0].position
                # Computing L_c for each debris following powerlaw
                new_row[2] = self._characteristic_length_distribution()
                # Computing A/M ratio for debris
                new_row[3] = self._AM_Ratio(new_row[2, 0])
                # Computing Area for each debris using L_c
                new_row[4] = self._compute_Area(new_row[2, 0])
                # Compute Mass using area and AM ratio
                new_row[5] = self._compute_mass(new_row[4, 0], new_row[3, 0])
                self._output = np.insert(self._output, -1, new_row, 0)
                output_mass = np.sum(self._output[:, 5, 0])

            # Remove the element that causes output mass to be too large now
            self._output = np.delete(self._output, -1, 0)

    def _compute_mass(self, area : float, AM_ratio : float) -> float:
        return area / AM_ratio

    def _compute_Area(self, characteristic_length : float) -> float:
        l_c_bound = 0.00167
        if characteristic_length < l_c_bound:
            factor = 0.540424
            return factor * characteristic_length * characteristic_length
        else:
            exponent = 2.0047077
            factor = 0.556945
            return factor * pow(characteristic_length, exponent)

    def _AM_Ratio(self, characteristic_length : float):
        log_l_c = np.log10(characteristic_length)
        if characteristic_length > 0.11:
            # Case bigger than 11 cm
            n1 = np.random.normal(
                mean_1(self._event.sat_type, log_l_c),
                sigma_1(self._event.sat_type, log_l_c),
            )
            n2 = np.random.normal(
                mean_2(self._event.sat_type, log_l_c),
                sigma_2(self._event.sat_type, log_l_c),
            )

            return pow(
                10.0,
                alpha(self._event.sat_type, log_l_c) * n1
                + (1 - alpha(self._event.sat_type, log_l_c)) * n2,
            )
        elif characteristic_length < 0.08:
            # Case smaller than 8 cm
            n = np.random.normal(mean_soc(log_l_c), sigma_soc(log_l_c))
            return pow(10.0, n)
        else:
            # Case between 8 cm and 11 cm
            n1 = np.random.normal(
                mean_1(self._event.sat_type, log_l_c),
                sigma_1(self._event.sat_type, log_l_c),
            )
            n2 = np.random.normal(
                mean_2(self._event.sat_type, log_l_c),
                sigma_2(self._event.sat_type, log_l_c),
            )
            n = np.random.normal(mean_soc(log_l_c), sigma_soc(log_l_c))

            y1 = pow(
                10.0,
                alpha(self._event.sat_type, log_l_c) * n1
                + (1.0 - alpha(self._event.sat_type, log_l_c)) * n2,
            )
            y0 = pow(10.0, n)

            return y0 + (characteristic_length - 0.08) * (y1 - y0) / (0.03)

    def _characteristic_length_distribution(self):
        # Sampling a value from uniform distribution
        y = np.random.uniform(0.0, 1.0)
        return power_law(
            self._min_characteristic_length,
            self._event.max_characteristic_length,
            self._event.lc_power_law_exponent,
            y,
        )
