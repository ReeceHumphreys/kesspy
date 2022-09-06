import numpy as np
from kessler_ext import SatellitePyWrapper, ExplosionEventPyWrapper, run_explosion


pos = np.array([0.0, 0.0, 0.0], np.float32)
vel = np.array([0.0, 0.0, 0, 0], np.float32)

sat = SatellitePyWrapper(pos, vel, 839.0, 0.01)
event = ExplosionEventPyWrapper(sat)
debris = run_explosion(event)

mean_area = np.mean(debris[:, 4, 0])
mean_mass = np.mean(debris[:, 5, 0])

print(f"{debris.shape[0]} Pieces of debris generated.")
print(f"Mean mass: {mean_mass}")
print(f"Mean area: {mean_area}")
