import numpy as np
from kesspy import Satellite, ExplosionEvent, run_explosion

pos = np.array([0.0, 0.0, 0.0], np.float32)
vel = np.array([0.0, 0.0, 0.0], np.float32)

sat = Satellite(pos, vel, 839.0)
event = ExplosionEvent(sat, 0.1)
debris = run_explosion(event)

rs = debris[:, 1, :].astype(np.float64)  # km
vs = debris[:, 6, :].astype(np.float64)  # km/s
char_les = debris[:, 2, :].astype(np.float64)  # m
ams = debris[:, 3, :].astype(np.float64)  # kg
areas = debris[:, 4, :].astype(np.float64)  # m^2
masses = debris[:, 5, :].astype(np.float64)  # kg

print("Average Characteristic len.:", np.mean(char_les[:, 0]))
print("Average A/M:", np.mean(ams[:, 0]))
print("Average Area:", np.mean(areas[:, 0]))
print("Average Mass:", np.mean(masses[:, 0]))

vel_mags = np.linalg.norm(vs, axis=1)
print("Average vel mag:", np.mean(vel_mags))
