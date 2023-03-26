import numpy as np
from kesspy import Satellite, ExplosionEvent, run_explosion

pos = np.array([0.0, 0.0, 0.0], np.float32)
vel = np.array([0.0, 0.0, 0, 0], np.float32)

sat = Satellite(pos, vel, 839.0, 0.000000000000001)
event = ExplosionEvent(sat)
debris = run_explosion(event)

mean_area = np.mean(debris[:, 4, 0])
mean_mass = np.mean(debris[:, 5, 0])

print(f"{debris.shape[0]} Pieces of debris generated.")
print(f"Mean mass: {mean_mass}")
print(f"Mean area: {mean_area}")
