import numpy as np


def distance_between(p1, p2):
    x1, y1 = p1
    x2, y2 = p2
    return np.math.sqrt((x1 - x2) ** 2 + (y1 - y2) ** 2)


def angle_between(origin, point):
    # calculate angle between point and origin with respect to 12 o'clock
    # returns value between 0 and 360
    x1, y1 = point
    x2, y2 = origin
    dX = x2 - x1
    dY = y2 - y1
    rads = np.math.atan2(-dX, dY)
    degrees = np.math.degrees(rads)
    if degrees >= 0:
        return degrees
    else:
        return 360 + degrees