from ..utils.utils import _M_to_nu
import numpy as np

# "m3 / (s2)"
mu_Earth = 3.986004418e14


class TLE:

    """
    Parse a TLE from its component line
    `name` is the "0th" line of the TLE
    """

    def __init__(self, name, line1, line2):
        self.name = name

        self.norad = line1[2:7].strip()
        self.classification = line1[7]
        self.int_desig = line1[9:17].strip()
        self.epoch_year = line1[18:20].strip()
        self.epoch_day = line1[20:32].strip()
        self.ballistic_coef = float(line1[33:43])
        self.dd_n = _parse_float(line1[44:52])
        self.bstar = _parse_float(line1[53:61])
        self.set_num = line1[64:68].strip()

        self.inc = float(line2[8:16])
        self.raan = float(line2[17:25])
        self.ecc = _parse_decimal(line2[26:33])
        self.argp = float(line2[34:42])
        self.M = float(line2[43:51])
        self.n = float(line2[52:63])
        self.rev_num = line2[63:68].strip()

        # Setting properties that need computing to `None`
        self._epoch = None
        self._a = None
        self._nu = None

    @property
    def epoch(self):
        """Epoch of the TLE."""
        if self._epoch is None:
            year = np.datetime64(self.epoch_year - 1970, "Y")
            day = np.timedelta64(
                int((self.epoch_day - 1) * 86400 * 10 ** 6), "us"
            )
            self._epoch = year + day
        return self._epoch

    @property
    def a(self):
        """Semi-major axis of TLE."""
        if self._a is None:
            self._a = (mu_Earth / (self.n * np.pi / 43200) ** 2) ** (
                1 / 3
            ) / 1000
        return self._a

    @property
    def nu(self):
        """True anomaly."""
        if self._nu is None:
            # Wrap M to [-pi, pi]
            M = (np.deg2rad(self.M) + np.pi) % (2 * np.pi) - np.pi
            ecc = self.ecc
            nu_rad = (_M_to_nu(M, ecc) + np.pi) % (2 * np.pi) - np.pi
            self._nu = np.rad2deg(nu_rad)
        return self._nu


def _conv_year(s):
    """Interpret a two-digit year string."""
    if isinstance(s, int):
        return s
    y = int(s)
    return y + (1900 if y >= 57 else 2000)


def _parse_decimal(s):
    """Parse a floating point with implicit leading dot.
    >>> _parse_decimal('378')
    0.378
    """
    return float("." + s)


def _parse_float(s):
    """Parse a floating point with implicit dot and exponential notation.
    >>> _parse_float(' 12345-3')
    0.00012345
    """
    return float(s[0] + "." + s[1:6] + "e" + s[6:8])
