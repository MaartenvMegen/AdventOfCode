from collections import defaultdict

from src.utility.PositionUtils import angle_between, distance_between


def rotate_and_kill(base, asteroids):
    kills_offset = 0
    kill_by_index = {}
    # KILL UNTIL NOTHING LEFT TO KILL!!!!
    while True:
        kills = get_possible_kills(base, asteroids, kills_offset)
        if len(kills) == 0:
            break

        kill_by_index.update(kills)
        kills_offset += len(kills)
        # VAPORIZE THEM!
        [asteroids.remove(kill) for kill in kills.values()]

    return kill_by_index


def get_possible_kills(base, asteroids, kill_offset):
    # Determine all asteroids angles and their distance to the base
    angles = defaultdict(list)
    for asteroid in asteroids:
        if asteroid != base:
            angle = angle_between(base, asteroid)
            distance = distance_between(base, asteroid)
            angles[angle].append((distance, asteroid))
    asteroids_sorted_by_angle = sorted(angles.items())

    # For each angle, find closest asteroid
    kills = defaultdict(int)
    for index, result in enumerate(asteroids_sorted_by_angle):
        closest_asteroid_to_base = sorted(result[1])[0]
        kills[index+kill_offset] = closest_asteroid_to_base[1]

    return kills


def get_best_asteroid(asteroids):
    possible_base_angle_count = defaultdict(int)
    for possible_base_location in asteroids:
        # Get a count of unique angles (directly visible asteroids) for each evaluated point of origin
        angles = set()
        for asteroid in asteroids:
            if asteroid != possible_base_location:
                angles.add(angle_between(possible_base_location, asteroid))
        possible_base_angle_count[possible_base_location] = len(angles)
    # Get angles sorted by amount of direct asteroid observations
    sorted_angles = sorted(possible_base_angle_count.items(), key=lambda kv: kv[1])
    asteroid, asteroid_count = sorted_angles[-1]

    return asteroid_count, asteroid


def get_asteroids(input_map):
    points = set()
    for y, line in enumerate(input_map):
        for x, character in enumerate(line):
            if character == "#":
                points.add((x, y))
    return points
