# KessPy

- [KessPy](#kesspy)
  - [Installation](#installation)
  - [Requirements](#requirements)
  - [Getting Started](#getting-started)
  - [Result Data Format](#result-data-format)
  - [Contributing](#contributing)
  - [License](#license)
  - [TODO](#todo)

KessPy is a Python library that uses the NASA Standard Breakup Model to simulate explosion and collision events in orbit, which is important for managing space debris and preventing collisions with other orbiting objects. It is a Python wrapper around Kessler.rs, a Rust implementation of the NASA Standard Breakup Model, which provides significant performance gains over purely Python-based implementations. More information about Kessler.rs can be found on its GitHub page at <https://github.com/reecehumphreys/kessler>.

**IMPORTANT**: Until this package reaches version 1.0, the API is subject to change and the accuracy of results cannot be guaranteed.

## Installation

KessPy runs on Python 3.6 or higher (Python 3.8 is recommended):
Currently the package is available using pip:

```shell
pip install kesspy
```

> _A conda distribution will be made available when the project is stable_

## Requirements

KessPy requires the following packages:

- numpy

## Getting Started

To get started with KessPy, you can use the following example code to simulate an explosion event and generate debris:

```python
import numpy as np
from kesspy import Satellite, ExplosionEvent, run_explosion

# Define the initial position, velocity, mass, characteristic length, and kind of a satellite.
pos = np.array([6.702e6, 0.0, 0.0], np.float32) # position vector [m] relative to Earth's center
vel = np.array([0.0, 7.666e3, 0.0], np.float32) # velocity vector [m/s]'
mass = 4.98e3; # [kg]
characteristic_length = 0.1; # [m]

# Create a new satellite with the given parameters
sat = Satellite(pos, vel, mass, characteristic_length)

# Create a new explosion event with the satellite
event = ExplosionEvent(sat)

# Run the simulation with the explosion event
debris = run_explosion(event)

# Print some statistics about the debris
mean_area = np.mean(debris[:, 4, 0])
mean_mass = np.mean(debris[:, 5, 0])

print(f"{debris.shape[0]} Pieces of debris generated.")
print(f"Mean mass: {mean_mass}")
print(f"Mean area: {mean_area}")
```

In this example, we define the initial position, velocity, mass, characteristic length, and type of a satellite. We then create a new Satellite object with these parameters, and use it to create an ExplosionEvent. Finally, we run the simulation with the run_explosion function, which generates debris data that we can analyze.

## Result Data Format

| index | data                       | type                              |
| ----- | -------------------------- | --------------------------------- |
| 0     | SatType (for internal use) | enum                              |
| 1     | position                   | np.array (, 3), containing floats |
| 2     | characteristic length      | float                             |
| 3     | area to mass ratio         | float                             |
| 4     | area                       | float                             |
| 5     | mass                       | float                             |
| 6     | velocity                   | np.array (, 3), containing floats |

> _The returned debris is an (n, 7, 3) numpy array. However, only the position and velocity use the third axis as those quanities are vectors._ >_All other fields have 3 copies of their respective data. This was done as a performance optimization for numpy_

## Contributing

If you find a bug or have a feature request, please open an issue on the KessPy
GitHub repository:

<https://github.com/reeceHumphreys/kesspy/issues>

If you'd like to contribute to the project, feel free to fork the repository and
submit a pull request.

## License

Kessler is distributed under the terms of the MIT license. See the LICENSE file
for details.

## TODO

- [ ] Update the documentation to reflect the new API
- [ ] Add tests for the new API
