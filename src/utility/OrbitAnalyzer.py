from collections import defaultdict


class OrbitAnalyzer:

    def __init__(self, orbit_input_spec):
        self.input = orbit_input_spec
        self.orbit_count = 0
        self.orbits_parsed = defaultdict(list)
        self.breadcrumbs = defaultdict(list)

    def analyse(self):
        for orbit_spec in self.input:
            around, in_orbit = orbit_spec.split(")")
            self.orbits_parsed[around].append(in_orbit)

        self.find_orbits("COM", 0, ["COM"])

    def find_orbits(self, orbitting_object, orbit_depth, origin):
        orbit_depth += 1
        for object_in_orbit in self.orbits_parsed[orbitting_object]:
            # print("current analyzing {} ; breadrumb {}".format(object_in_orbit, origin))
            self.breadcrumbs[object_in_orbit] = origin
            new_origin = origin.copy()
            new_origin.append(object_in_orbit)
            self.orbit_count += orbit_depth
            self.find_orbits(object_in_orbit, orbit_depth, new_origin)

    def get_transfers(self, origin, destination):
        common_ancestors = [element for element in self.breadcrumbs[origin] if element in self.breadcrumbs[destination]]
        ancestor = common_ancestors[-1]
        distance_ancestor_origin = len(self.breadcrumbs[origin]) - self.breadcrumbs[origin].index(ancestor) - 1
        distance_ancestor_destination = len(self.breadcrumbs[destination]) - self.breadcrumbs[origin].index(ancestor) - 1

        print("You travel distance will be")
        return distance_ancestor_origin + distance_ancestor_destination

    def get_orbit_count(self):
        return self.orbit_count
