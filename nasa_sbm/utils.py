def power_law(x0, x1, n, y):
    step = pow(x1, n + 1) - pow(x0, n + 1) * y + pow(x0, n + 1)
    return pow(step, 1 / (n + 1))
