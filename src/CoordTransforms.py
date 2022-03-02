import numpy as np
from numba import njit as jit, prange
from numpy.core.umath import cos, sin, sqrt
from numpy.linalg import norm

"""
Converting from Keplerian to Cartesian
--------------------------------------
Helpful links:
    https://downloads.rene-schwarz.com/download/M001-Keplerian_Orbit_Elements_to_Cartesian_State_Vectors.pdf
    https://gitlab.eng.auburn.edu/evanhorn/orbital-mechanics/blob/a850737fcf4c43e295e79decf2a3a88acbbba451/Homework1/kepler.py

Notes: Code was modified from Poliastro source, elements.py
"""

mu = 398600.4418  # km^3s^-2

@jit
def rotation_matrix(angle, axis):
    c = cos(angle)
    s = sin(angle)

    if axis == 0:
        return np.array([[1.0, 0.0, 0.0], [0.0, c, -s], [0.0, s, c]])
    elif axis == 1:
        return np.array([[c, 0.0, s], [0.0, 1.0, 0.0], [s, 0.0, c]])
    elif axis == 2:
        return np.array([[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]])
    else:
        raise ValueError("Invalid axis: must be one of 'x', 'y' or 'z'")


@jit
def rv_pqw(k, p, ecc, nu):
    pqw = np.array([[cos(nu), sin(nu), 0], [-sin(nu), ecc + cos(nu), 0]]
                   ) * np.array([[p / (1 + ecc * cos(nu))], [sqrt(k / p)]])
    return pqw


@jit
def coe_rotation_matrix(inc, raan, argp):
    """Create a rotation matrix for coe transformation"""
    r = rotation_matrix(raan, 2)
    r = r @ rotation_matrix(inc, 0)
    r = r @ rotation_matrix(argp, 2)
    return r


@jit
def coe2rv(k, p, ecc, inc, raan, argp, nu):

    pqw = rv_pqw(k, p, ecc, nu)
    r, v = rv_pqw(k, p, ecc, nu)
    rm = coe_rotation_matrix(inc, raan, argp)
    ijk = pqw @ rm.T

    return ijk

# ks = np.array([a, e_mag, i, Omega, omega, M, nu, p_semi, T, E])


@jit(parallel=True)
def coe2rv_many_new(state, mu=mu):
    inc = np.deg2rad(state[2, :])
    raan = np.deg2rad(state[3, :])
    argp = np.deg2rad(state[4, :])
    nu = np.deg2rad(state[6, :])
    p = state[7, :]
    ecc = state[1, :]

    n = nu.shape[0]
    rr = np.zeros((n, 3), dtype=np.float64)
    vv = np.zeros((n, 3), dtype=np.float64)

    for i in prange(n):
        rr[i, :], vv[i, :] = (
            coe2rv(mu, p[i], ecc[i], inc[i], raan[i], argp[i], nu[i]))

    return rr, vv


@jit(parallel=True)
def coe2rv_many(k, p, ecc, inc, raan, argp, nu):
    inc = np.deg2rad(inc)
    raan = np.deg2rad(raan)
    argp = np.deg2rad(argp)
    nu = np.deg2rad(nu)

    n = nu.shape[0]
    rr = np.zeros((n, 3), dtype=np.float64)
    vv = np.zeros((n, 3), dtype=np.float64)

    for i in prange(n):
        rr[i, :], vv[i, :] = (
            coe2rv(k, p[i], ecc[i], inc[i], raan[i], argp[i], nu[i]))

    return rr, vv


"""
Converting from Cartesian to Keplerian
--------------------------------------
"""


def rv2coe(r, v, mu):
    '''
    Converts a position, `r`, and a velocity, `v` to the set of keplerian
    elements.
    '''
    """
    Parameters
    ----------
    r: array (3, n)
       Position of the body in 3 dim. Measured using center of Earth
       as origin. (m)
    v: array (3, n)
       Velocity of the body in 3 dim relative to Earth. (m / s)

    Returns
    -------
    ks: array (9, n)
        An array containing all of the keplerian elements + extra useful info.
        a: Float
        e: Float
        i: Float
        Omega: Float
        omega: Float
        nu: Float
        p_semi: Float
        T: Float
    """

    def testAngle(test, angle):
        """Checks test for sign and returns corrected angle"""
        angle *= 180. / np.pi
        index = test < 0
        angle[index] = 360. - angle[index]
        return angle

    r_hat = np.divide(r, norm(r, axis=1)[:, None])

    # Orbital momentum vector, p
    p = np.cross(r, v)

    # Eccentricty vector, e, and magnitude, e_mag (used freq)
    e = (np.cross(v, p) / mu) - r_hat
    e_mag = norm(e, axis=1)

    # Longitude of the ascending node, Omega
    Omega_hat = np.cross(np.array([0, 0, 1])[None, :], p)
    Omega = np.arccos(Omega_hat[:, 0] / norm(Omega_hat, axis=1))
    Omega = testAngle(Omega_hat[:, 1], Omega)

    # Argument of periapsis, omega
    omega = np.arccos(np.sum(Omega_hat * e, axis=1) /
                      (norm(Omega_hat, axis=1) * norm(e, axis=1)))
    B = e[:, 2] < 0
    omega[B] = 2 * np.pi - omega[B]
    omega *= 180. / np.pi

    # True Anomaly, nu
    nu = np.arccos(np.sum(e * r, axis=1) / (norm(e, axis=1) * norm(r, axis=1)))
    B = np.sum(r * v, axis=1) < 0
    nu[B] = 2 * np.pi - nu[B]
    nu *= 180. / np.pi

    # Inclination, i
    i = np.arccos(p[:, 2] / norm(p, axis=1)) * 180. / np.pi

    # Eccentric anomaly, E
    E = 2 * np.arctan(np.tan(np.deg2rad(nu) / 2) /
                      np.sqrt((1 + e_mag) / (1 - e_mag)))

    # Mean anomaly, M
    M = np.mod(E - e_mag * np.sin(E), 2 * np.pi)
    M *= 180. / np.pi

    # Semi-Major axis, a
    R = norm(r, axis=1)
    V = norm(v, axis=1)
    a = 1 / ((2 / R) - (V * V / mu))

    # Semi-parmeter, p_semi
    p_semi = norm(p, axis=1)**2 / mu

    # Orbital period
    T = 2 * np.pi * np.sqrt(a**3 / mu)

    # Keplerian State + Extra info
    ks = np.array([a, e_mag, i, Omega, omega, M, nu, p_semi, T, E])

    return ks
