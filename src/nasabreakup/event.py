import numpy as np
from .configuration import SatType


class Collision():
    @property
    def lc_power_law_exponent(self) -> float:
        """
        Gets the exponents used in the characteristic length power law
        :py:meth:`odap.model.BreakupModel._characteristic_length_distribution`.
        """
        return -2.71

    @property
    def delta_velocity_offset(self) -> list[float]:
        """
        Gets the offset factors used in determining the change in velocity for each
        fragment.
        """
        return [0.9, 2.9]

    @property
    def max_characteristic_length(self) -> float:
        """
        Gets the largest characteristic length possible for the fragmentation event.
        For collisions, this is the characteristic length of the more massive satellite.
        """
        return self._max_characteristic_length

    @property
    def sat_type(self) -> SatType:
        """
        Gets the satellite type for the fragmentation event. In the case that either
        of the satellites involved in the collision are rocket bodies, the type is
        rocket body. Otherwise, the default is a spacecraft
        """
        return self._sat_type

    @property
    def input_mass(self) -> float:
        """
        Gets the input mass for the fragmentation event. For collisions, this is the sum
        of the massses of both satellites.
        """
        return self._input_mass

    def fragment_count(self, satellites: np.ndarray, min_characteristic_length : float):
        """
        Determines the number of debris fragments produced by the fragmentation event.
        Noteably, this quantity can change if mass conservation is being enforced.

        Parameters
        ----------
        satellites : np.array([Satellite])
            The satellites involved in the collision. Maximum of two.
        min_characteristic_length : float
            The smallest characteristic length size we wish to generate.
            Note, the smaller the min_characteristic_length, the longer the simulation
            will take to run and the more debris will be generated.

        Returns
        -------
        int
            The number of debris generated,
        """

        if min_characteristic_length == 0:
            raise ZeroDivisionError

        satellite_1 = satellites[0]
        satellite_2 = satellites[1]
        self._max_characteristic_length = [
            satellite_1.characteristic_length,
            satellite_2.characteristic_length,
        ]
        self._sat_type = SatType.soc

        if satellite_1.type == SatType.rb or satellite_2.type == SatType.rb:
            self._sat_type = SatType.rb

        # satellite_1 should be the bigger satellite
        if (
            satellite_2.characteristic_length
            > satellite_1.characteristic_length
        ):
            satellite_1, satellite_2 = satellite_2, satellite_1

        self._input_mass = satellite_1.mass + satellite_2.mass
        mass = 0

        # The Relative Collision Velocity
        delta_velocity = np.linalg.norm(
            satellite_1.velocity - satellite_2.velocity
        )

        catastrophic_ratio = (
            satellite_2.mass * delta_velocity * delta_velocity
        ) / (2.0 * satellite_1.mass * 1000.0)

        if catastrophic_ratio < 40.0:
            self._is_catastrophic = False
            mass = satellite_2.mass * delta_velocity / 1000.0
        else:
            self._is_catastrophic = True
            mass = satellite_1.mass + satellite_2.mass

        return int(
            0.1 * pow(mass, 0.75) * pow(min_characteristic_length, -1.71)
        )


class Explosion():
    @property
    def lc_power_law_exponent(self) -> float:
        """
        Gets the exponents used in the characteristic length power law
        :py:meth:`odap.model.BreakupModel._characteristic_length_distribution`.
        """
        return -2.6

    @property
    def delta_velocity_offset(self) -> list[float]:
        """
        Gets the offset factors used in determining the change in velocity for each
        fragment.
        """
        return [0.2, 1.85]

    @property
    def max_characteristic_length(self) -> float:
        """
        Gets the largest characteristic length possible for the fragmentation event.
        For explosions, this is the characteristic length of the input satellite.
        """
        return self._max_characteristic_length

    @property
    def sat_type(self) -> SatType:
        """
        Gets the satellite type for the fragmentation event.
        """
        return self._sat_type

    @property
    def input_mass(self) -> float:
        """
        Gets the input mass for the fragmentation event. For explosions, this is the
        mass of the input satellite.
        """
        return self._input_mass

    def fragment_count(self, satellites: np.ndarray, min_characteristic_length : float) -> np.ndarray:
        """
        Determines the number of debris fragments produced by the fragmentation event.
        Noteably, this quantity can change if mass conservation is being enforced.

        Parameters
        ----------
        satellites : np.array([Satellite])
            The satellites involved in the fragmentation event. Maximum of one.
        min_characteristic_length : float
            The smallest characteristic length size we wish to generate.
            Note, the smaller the min_characteristic_length, the longer the simulation
            will take to run and the more debris will be generated.

        Returns
        -------
        int
            The number of debris generated.
        """
        if min_characteristic_length == 0:
            raise ZeroDivisionError

        satellite = satellites[0]
        self._max_characteristic_length = satellite.characteristic_length
        self._sat_type = satellite.type
        self._input_mass = satellite.mass

        S = 1
        return int(6 * S * (min_characteristic_length) ** (-1.6))
