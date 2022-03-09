# NASA Breakup Model in Python

<div style="display:flex; flex-direction: row; justify-content: center; align-items: center">
  <a href="https://nasa-breakup-model-python.readthedocs.io/en/latest/">
    <img alt="docs" src="https://img.shields.io/readthedocs/nasa-breakup-model-python" target="_blank">
  </a>
  <a href="https://github.com/ReeceHumphreys/NASA-breakup-model-python/blob/main/LICENSE">
    <img alt="License: MIT" src="https://img.shields.io/github/license/ReeceHumphreys/NASA-breakup-model-python" target="_blank" />
  </a>
  <img src="https://img.shields.io/lgtm/grade/python/github/ReeceHumphreys/NASA-breakup-model-python" target="_blank"/>
  <a href="https://pypi.org/project/nasabreakup/">
    <img src="https://img.shields.io/pypi/v/nasabreakup"target="_blank"/>
  </a>
</div>

<br>
NASA Standard Breakup Model in Python is a Python library for simulating explosion and collision events in orbit using the NASA Standard Breakup Model. The breakup model was implemented based on the following works: NASA’s new breakup model of evolve 4.0 (Johnson et al.), and Proper Implementation of the 1998 NASA Breakup Model (Krisko et al.).

## Installation
NASA Breakup Model in Python runs on Python 3.6 or higher (Python 3.8 is recommended):
Currently the package is available using pip:
```
pip install nasabreakup
```
>*A conda distribution will be made available when the project is stable*

## Getting Started

To use NASA Breakup Model in Python, you must first create a .yaml file to configure the simulation.
This file has three required fields, the minimum characteristic length, the [simulation type](https://nasa-breakup-model-python.readthedocs.io/en/latest/_autosummary/nasabreakup.configuration.SimulationType.html),
and the [satellite type](https://nasa-breakup-model-python.readthedocs.io/en/latest/_autosummary/nasabreakup.configuration.SatType.html)
involved in the fragmentation event.

Secondly, you must provide an implementation of [Satellite](https://nasa-breakup-model-python.readthedocs.io/en/latest/_autosummary/nasabreakup.satellite.Satellite.html).

Once, you have those two criterion met you can perform the simulation as follows:

```python
from nasabreakup.configuration import SimulationConfiguration
from nasabreakup.model import BreakupModel

config = SimulationConfiguration('data/simulation_config.yaml')
event  = BreakupModel(config, np.array([sat]))
debris = event.run()
```
> An example configuration.yaml and Satellite implementation has been provided in `examples`

## Resulting Data Format

| index | data                       | type                               |
|-------|----------------------------|------------------------------------|
| 0     | SatType (for internal use) | enum                               |
| 1     | position                   | np.array (, 3), containing floats  |
| 2     | characteristic length      | float                              |
| 3     | area to mass ratio         | float                              |
| 4     | area                       | float                              |
| 5     | mass                       | float                              |
| 6     | velocity                   | np.array (, 3), containing floats  |

>*The returned debris is an (n, 7, 3) numpy array. However, only the position and velocity use the third axis as those quanities are vectors.*
>*All other fields have 3 copies of their respective data. This was done as a performance optimization for numpy*

## Documentation
- [Read the Docs](https://nasa-breakup-model-python.readthedocs.io/en/latest/)
