from .configuration import SatType


def distribution_constant(
    log_L_c, lower, upper, lower_return, upper_return, mid_function
):
    if log_L_c <= lower:
        return lower_return
    elif log_L_c >= upper:
        return upper_return
    else:
        return mid_function(log_L_c)

def alpha(sat_type, log_L_c):
    if sat_type == SatType.rb:
        return distribution_constant(
            log_L_c,
            -1.4,
            0.0,
            1.0,
            0.5,
            lambda log_L_c: 1.0 - 0.3571 * (log_L_c + 1.4),
        )
    else:
        return distribution_constant(
            log_L_c,
            -1.95,
            0.55,
            0.0,
            1.0,
            lambda log_L_c: 0.3 + 0.4 * (log_L_c + 1.2),
        )


# Handles RB and SC case
def mean_1(sat_type, log_L_c):
    if sat_type == SatType.rb:
        return distribution_constant(
            log_L_c,
            -0.5,
            0.0,
            -0.45,
            -0.9,
            lambda log_L_c: -0.45 - 0.9 * (log_L_c + 0.5),
        )
    else:
        return distribution_constant(
            log_L_c,
            -1.1,
            0.0,
            -0.6,
            -0.95,
            lambda log_L_c: -0.6 - 0.318 * (log_L_c + 1.1),
        )


# Handles RB and SC case


def sigma_1(sat_type, log_L_c):
    if sat_type == SatType.rb:
        return 0.55
    else:
        return distribution_constant(
            log_L_c,
            -1.3,
            -0.3,
            0.1,
            0.3,
            lambda log_L_c: 0.1 + 0.2 * (log_L_c + 1.3),
        )


# Handles RB and SC case


def mean_2(sat_type, log_L_c):
    if sat_type == SatType.rb:
        return -0.9
    else:
        return distribution_constant(
            log_L_c,
            -0.7,
            -0.1,
            -1.2,
            -2.0,
            lambda log_L_c: -1.2 - 1.333 * (log_L_c + 0.7),
        )


# Handles RB and SC case


def sigma_2(sat_type, log_L_c):
    if sat_type == SatType.rb:
        return distribution_constant(
            log_L_c,
            -1.0,
            0.1,
            0.28,
            0.1,
            lambda log_L_c: -0.28 - 0.1636 * (log_L_c + 1.0),
        )

    else:
        return distribution_constant(
            log_L_c,
            -0.5,
            -0.3,
            0.5,
            0.3,
            lambda log_L_c: 0.5 - (log_L_c + 0.5),
        )


def mean_soc(log_L_c):
    return distribution_constant(
        log_L_c,
        -1.75,
        -1.25,
        -0.3,
        -1.0,
        lambda log_L_c: -0.3 - 1.4 * (log_L_c + 1.75),
    )


def sigma_soc(log_L_c):
    return 0.2 if log_L_c <= -3.5 else 0.2 + 0.1333 * (log_L_c + 3.5)

def power_law(x0, x1, n, y):
    step = pow(x1, n + 1) - pow(x0, n + 1) * y + pow(x0, n + 1)
    return pow(step, 1 / (n + 1))