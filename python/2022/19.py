import sys
from typing import NamedTuple
from collections import deque
from math import prod

class Materials(NamedTuple):
    geode: int = 0
    obsidian: int = 0
    clay: int = 0
    ore: int = 0

def parse(text):
    res = {}
    for row in text.strip().split('\n'):
        parts = row.split()
        key = int(parts[1][:-1])
        res[key] = Materials(
            ore = Materials(ore = int(parts[6])),
            clay = Materials(ore = int(parts[12])),
            obsidian = Materials(ore = int(parts[18]), clay=int(parts[21])),
            geode = Materials(ore = int(parts[27]), obsidian=int(parts[30])))
    return list(sorted(res.items()))

def can_afford(mat: Materials, recipe: Materials):
    return all(m >= i for m, i in zip(mat, recipe))

def max_geode(minutes_left: int, robots: Materials, material: Materials, bp: Materials):
    max_cost = Materials(*(max(v) for v in zip(*bp)))._asdict()
    recipes = bp._asdict()

    fringe = deque([(minutes_left, robots, material, False)])
    best = 0
    while fringe:
        minutes_left, robots, material, skipped_building = fringe.popleft()
        if best < material.geode:
            best = material.geode
            #print(best)
        if minutes_left == 0:
            continue
        collected = Materials(*(had + prod for had, prod in zip(material, robots)))
        could_build = False
        for purchase in ['geode', 'obsidian', 'clay', 'ore']:
            if skipped_building:
                could_build = True
                break
            recipe = recipes[purchase]
            robot_count = getattr(robots, purchase)
            if purchase != 'geode' and robot_count >= max_cost[purchase]:
                continue
            if can_afford(material, recipe):
                could_build = True#could_build or (purchase == 'geode')
                nxt_robots = robots._replace(**{purchase: robot_count + 1})
                nxt_mat = Materials(*(had - spent for had, spent in zip(collected, recipe)))
                fringe.append((minutes_left - 1, nxt_robots, nxt_mat, False))
                if purchase == 'geode' or purchase == 'obsidian':
                    break
        fringe.append((minutes_left - 1, robots, collected, could_build))
    return best

def max_geode(minutes_left: int, robots: Materials, material: Materials, bp: Materials):
    max_cost = Materials(*(max(v) for v in zip(*bp)))._asdict()
    recipes = bp._asdict()

    fringe = deque([(minutes_left, robots, material, False, tuple())])
    best = 0
    while fringe:
        minutes_left, robots, material, skipped_building, path = fringe.popleft()
        #print(minutes_left, material, robots)
        if best < material.geode:
            best = material.geode
            print(best, robots, material, minutes_left, '....', path)
        if minutes_left <= 0:
            continue

        for purchase in ['geode', 'obsidian', 'clay', 'ore']:
            recipe = recipes[purchase]
            robot_count = getattr(robots, purchase)
            if purchase != 'geode' and robot_count >= max_cost[purchase]:
                continue
            if not all(robot_present for robot_present, ingredent_needed in zip(robots, recipe) if ingredent_needed):
                continue
            time_to_afford = max((i-m) // r for r, m, i in zip(robots, material, recipe) if i)
            if time_to_afford > minutes_left or time_to_afford < 0:
                total = material.geode + robots.geode*minutes_left
                if total > best:
                    best = total
                    print(best, path, "< early stop?")
                continue
            assert time_to_afford >= 0, time_to_afford
            #print(purchase, time_to_afford, minutes_left - time_to_afford)
            nxt_mat = Materials(*((m + r*(time_to_afford+1) - i) for r, m, i in zip(robots, material, recipe)))
            assert all(i >= 0 for i in nxt_mat), nxt_mat
            nxt_robots = robots._replace(**{purchase: robot_count + 1})
            fringe.append((minutes_left - time_to_afford-1, nxt_robots, nxt_mat, False, path + ((minutes_left-time_to_afford-1, purchase), )))
            #if purchase == 'geode' or purchase == 'obsidian': break
    print('...', best)
    return best

from heapq import heappop, heappush
from math import ceil
def max_geode(minutes_left: int, robots: Materials, material: Materials, bp: Materials):
    max_cost = Materials(*(max(v) for v in zip(*bp)))._asdict()
    recipes = bp._asdict()

    best = 0
    fringe = [(minutes_left, robots, material)]
    while fringe:
        minutes_left, robots, material = fringe.pop()
        if material.geode > best:
            best = material.geode
            print('best...', best, minutes_left, robots, len(fringe))
        for purchase in ['geode', 'obsidian', 'clay', 'ore']:
            recipe = recipes[purchase]
            robot_count = getattr(robots, purchase)
            if purchase != 'geode' and robot_count >= max_cost[purchase]:
                continue
            if not all(robot_present for robot_present, ingredent_needed in zip(robots, recipe) if ingredent_needed):
                continue
            time_to_afford = max(ceil((i-m) / r) for r, m, i in zip(robots, material, recipe) if i)
            if time_to_afford + 1> minutes_left: continue
            if time_to_afford < 0:
                time_to_afford = 0
            wait_time = time_to_afford + 1
            nxt_minutes_left = minutes_left - wait_time
            if wait_time > minutes_left: continue
            #print(f'can build {purchase:5} in {time_to_afford:2} minutes')
            nxt_material = Materials(*((m + r*(wait_time) - i) for r, m, i in zip(robots, material, recipe)))
            nxt_robots = robots._replace(**{purchase: robot_count + 1})
            assert all(i >= 0 for i in nxt_material), (nxt_material, purchase, recipe, material, time_to_afford)
            fringe.append((nxt_minutes_left, nxt_robots, nxt_material))
            #if purchase == 'geode' or purchase == 'obsidian': break
    return best

# this work, don't touch.
def max_geode(minutes_left: int, robots: Materials, material: Materials, bp: Materials):
    max_cost = Materials(*(max(v) for v in zip(*bp)))._asdict()
    recipes = bp._asdict()

    best = 0
    fringe = [(minutes_left, robots, material)]
    while fringe:
        minutes_left, robots, material = fringe.pop()
        bare_minimum = material.geode + (robots.geode * max(minutes_left, 0))
        abs_maximum = (minutes_left * (minutes_left + 1) // 2) + material.geode + robots.geode * minutes_left
        if bare_minimum > best:
            best = bare_minimum
            print('best...', best, minutes_left, robots, len(fringe))
        if minutes_left < 0:
            continue
        if abs_maximum < best:
            continue
        for purchase in ['geode', 'obsidian', 'clay', 'ore']:
            recipe = recipes[purchase]
            robot_count = getattr(robots, purchase)
            if purchase != 'geode' and robot_count >= max_cost[purchase]:
                continue
            if not all(robot_present for robot_present, ingredent_needed in zip(robots, recipe) if ingredent_needed):
                continue
            time_to_afford = max(ceil((i-m) / r) for r, m, i in zip(robots, material, recipe) if i)
            if time_to_afford + 1> minutes_left: continue
            if time_to_afford < 0:
                time_to_afford = 0
            wait_time = time_to_afford + 1
            nxt_minutes_left = minutes_left - wait_time
            if wait_time > minutes_left: continue
            nxt_material = Materials(*((m + r*(wait_time) - i) for r, m, i in zip(robots, material, recipe)))
            nxt_robots = robots._replace(**{purchase: robot_count + 1})
            assert all(i >= 0 for i in nxt_material), (nxt_material, purchase, recipe, material, time_to_afford)
            fringe.append((nxt_minutes_left, nxt_robots, nxt_material))
    return best

def lower_bound(minutes_left, robots, material, recipe):
    for _ in range(minutes_left):
        if not all(r for r, i in zip(robots, recipe) if i):
            return material.geode
        nxt_rob = robots
        if all(m >= i for m,i in zip(material, recipe)):
            nxt_rob._replace(geode = nxt_rob.geode + 1)
            material = Materials(*(m - i for m, i in zip(material, recipe)))
        material = Materials(*(m + r for m, r in zip(material, robots)))
        robots = nxt_rob
    return material.geode

def upper_bound(minutes_left, robots, material, bp):
    return (minutes_left * (minutes_left + 1) // 2) + material.geode + robots.geode * minutes_left

def upper_bound(minutes_left, robots, material, bp):
    for _ in range(minutes_left):
        built = list(robots)
        for k, recipe in enumerate(bp):
            if all(m >= i for m, i in zip(material, recipe)):
                built[k] += 1
        material = Materials(*(m + r for m, r in zip(material, robots)))
        robots = Materials(*built)
    return material.geode


MAX = [0]
def max_geode(minutes_left: int, robots: Materials, material: Materials, bp: Materials):
    max_cost = Materials(*(max(v) for v in zip(*bp)))._asdict()
    recipes = bp._asdict()

    best = 0
    fringe = [((0, 0), (minutes_left, robots, material))]
    while fringe:
        MAX[0] = max(MAX[0], len(fringe))
        heu, (minutes_left, robots, material) = heappop(fringe)
        bare_minimum = material.geode + (robots.geode * max(minutes_left, 0))
        #abs_maximum = (minutes_left * (minutes_left + 1) // 2) + material.geode + robots.geode * minutes_left
        abs_maximum = -heu[1]
        if bare_minimum > best:
            best = bare_minimum
            #print('best...', best, minutes_left, robots, len(fringe))
        if minutes_left < 0:
            continue
        if abs_maximum < best:
            continue
        for purchase in ['geode', 'obsidian', 'clay', 'ore']:
            recipe = recipes[purchase]
            robot_count = getattr(robots, purchase)
            if purchase != 'geode' and robot_count >= max_cost[purchase]:
                continue
            if not all(robot_present for robot_present, ingredent_needed in zip(robots, recipe) if ingredent_needed):
                continue
            time_to_afford = max(ceil((i-m) / r) for r, m, i in zip(robots, material, recipe) if i)
            if time_to_afford + 1> minutes_left: continue
            if time_to_afford < 0:
                time_to_afford = 0
            wait_time = time_to_afford + 1
            nxt_minutes_left = minutes_left - wait_time
            if wait_time > minutes_left: continue
            nxt_material = Materials(*((m + r*(wait_time) - i) for r, m, i in zip(robots, material, recipe)))
            nxt_robots = robots._replace(**{purchase: robot_count + 1})
            assert all(i >= 0 for i in nxt_material), (nxt_material, purchase, recipe, material, time_to_afford)
            #fringe.append((nxt_minutes_left, nxt_robots, nxt_material))
            nxt_abs_maximum = (nxt_minutes_left * (nxt_minutes_left + 1) // 2) + nxt_material.geode + nxt_robots.geode * nxt_minutes_left
            nxt_abs_maximum = upper_bound(nxt_minutes_left, nxt_robots, nxt_material, bp)
            nxt_heu = (-nxt_robots.geode, -nxt_abs_maximum) # 16...
            if nxt_abs_maximum < best: continue

            # nxt_min = lower_bound(nxt_minutes_left, nxt_robots, nxt_material, bp.geode)
            # if nxt_min > best:
            #     best = nxt_min

            heappush(fringe, (nxt_heu, (nxt_minutes_left, nxt_robots, nxt_material)))
    return best

MAX = [0]
def max_geode(minutes_left: int, robots: Materials, material: Materials, bp: Materials):
    max_cost = Materials(*(max(v) for v in zip(*bp)))._asdict()
    recipes = bp._asdict()

    best = 0
    fringe = [((0, 0), (minutes_left, robots, material))]
    while fringe:
        MAX[0] = max(MAX[0], len(fringe))
        heu, (minutes_left, robots, material) = heappop(fringe)
        bare_minimum = material.geode + (robots.geode * max(minutes_left, 0))
        #abs_maximum = (minutes_left * (minutes_left + 1) // 2) + material.geode + robots.geode * minutes_left
        abs_maximum = -heu[1]
        if bare_minimum > best:
            best = bare_minimum
            #print('best...', best, minutes_left, robots, len(fringe))
        if minutes_left < 0:
            continue
        if abs_maximum < best:
            continue
        for purchase in ['geode', 'obsidian', 'clay', 'ore']:
            recipe = recipes[purchase]
            robot_count = getattr(robots, purchase)
            if purchase != 'geode' and robot_count >= max_cost[purchase]:
                continue
            if not all(robot_present for robot_present, ingredent_needed in zip(robots, recipe) if ingredent_needed):
                continue
            time_to_afford = max(ceil((i-m) / r) for r, m, i in zip(robots, material, recipe) if i)
            if time_to_afford + 1> minutes_left: continue
            if time_to_afford < 0:
                time_to_afford = 0
            wait_time = time_to_afford + 1
            nxt_minutes_left = minutes_left - wait_time
            if wait_time > minutes_left: continue
            nxt_material = Materials(*((m + r*(wait_time) - i) for r, m, i in zip(robots, material, recipe)))
            nxt_robots = robots._replace(**{purchase: robot_count + 1})
            assert all(i >= 0 for i in nxt_material), (nxt_material, purchase, recipe, material, time_to_afford)
            #fringe.append((nxt_minutes_left, nxt_robots, nxt_material))
            nxt_abs_maximum = (nxt_minutes_left * (nxt_minutes_left + 1) // 2) + nxt_material.geode + nxt_robots.geode * nxt_minutes_left
            nxt_abs_maximum = upper_bound(nxt_minutes_left, nxt_robots, nxt_material, bp)
            nxt_heu = (-nxt_robots.geode, -nxt_abs_maximum) # 16...
            if nxt_abs_maximum < best: continue

            # nxt_min = lower_bound(nxt_minutes_left, nxt_robots, nxt_material, bp.geode)
            # if nxt_min > best:
            #     best = nxt_min

            heappush(fringe, (nxt_heu, (nxt_minutes_left, nxt_robots, nxt_material)))
    return best

def max_geode(minutes_left: int, robots: Materials, material: Materials, bp: Materials):
    max_cost = Materials(*(max(v) for v in zip(*bp)))._asdict()
    recipes = bp._asdict()

    best = 0
    fringe = [((0, 0), (minutes_left, robots, material))]
    while fringe:
        MAX[0] = max(MAX[0], len(fringe))
        heu, (minutes_left, robots, material) = heappop(fringe)
        bare_minimum = material.geode + (robots.geode * max(minutes_left, 0))
        #abs_maximum = (minutes_left * (minutes_left + 1) // 2) + material.geode + robots.geode * minutes_left
        abs_maximum = -heu[1]
        if bare_minimum > best:
            best = bare_minimum
            #print('best...', best, minutes_left, robots, len(fringe))
        if minutes_left < 0:
            continue
        if abs_maximum < best:
            #break
            continue
        for purchase in ['geode', 'obsidian', 'clay', 'ore']:
            recipe = recipes[purchase]
            robot_count = getattr(robots, purchase)
            if purchase != 'geode' and robot_count >= max_cost[purchase]:
                continue
            if not all(robot_present for robot_present, ingredent_needed in zip(robots, recipe) if ingredent_needed):
                continue
            time_to_afford = max(ceil((i-m) / r) for r, m, i in zip(robots, material, recipe) if i)
            if time_to_afford + 1> minutes_left: continue
            if time_to_afford < 0:
                time_to_afford = 0
            wait_time = time_to_afford + 1
            nxt_minutes_left = minutes_left - wait_time
            if wait_time > minutes_left: continue
            nxt_material = Materials(*((m + r*(wait_time) - i) for r, m, i in zip(robots, material, recipe)))
            nxt_robots = robots._replace(**{purchase: robot_count + 1})
            assert all(i >= 0 for i in nxt_material), (nxt_material, purchase, recipe, material, time_to_afford)
            #fringe.append((nxt_minutes_left, nxt_robots, nxt_material))
            nxt_abs_maximum = (nxt_minutes_left * (nxt_minutes_left + 1) // 2) + nxt_material.geode + nxt_robots.geode * nxt_minutes_left
            nxt_abs_maximum = upper_bound(nxt_minutes_left, nxt_robots, nxt_material, bp)
            nxt_heu = (-nxt_robots.geode, -nxt_abs_maximum) # 16...
            if nxt_abs_maximum < best: continue

            # nxt_min = lower_bound(nxt_minutes_left, nxt_robots, nxt_material, bp.geode)
            # if nxt_min > best:
            #     best = nxt_min

            heappush(fringe, (nxt_heu, (nxt_minutes_left, nxt_robots, nxt_material)))
    return best

def max_geode(minutes_left: int, robots: Materials, material: Materials, bp: Materials):
    max_cost = Materials(*(max(v) for v in zip(*bp)))._asdict()
    recipes = bp._asdict()

    def build(state):
        (minutes_left, robots, material) = state
        for purchase in ['geode', 'obsidian', 'clay', 'ore']:
            recipe = recipes[purchase]
            robot_count = getattr(robots, purchase)
            if purchase != 'geode' and robot_count >= max_cost[purchase]:
                continue
            if not all(robot_present for robot_present, ingredent_needed in zip(robots, recipe) if ingredent_needed):
                continue
            time_to_afford = max(ceil((i-m) / r) for r, m, i in zip(robots, material, recipe) if i)
            wait_time = max(0, time_to_afford) + 1
            if wait_time > minutes_left: continue
            nxt_minutes_left = minutes_left - wait_time
            nxt_material = Materials(*((m + r*(wait_time) - i) for r, m, i in zip(robots, material, recipe)))
            nxt_robots = robots._replace(**{purchase: robot_count + 1})
            assert all(i >= 0 for i in nxt_material), (nxt_material, purchase, recipe, material, time_to_afford)
            assert all(i >= 0 for i in nxt_robots), (nxt_material, purchase, recipe, material, time_to_afford)
            assert nxt_minutes_left >= 0, (nxt_material, purchase, recipe, material, time_to_afford)
            yield (nxt_minutes_left, nxt_robots, nxt_material)

    def upper_bound(state):
        (minutes_left, robots, material) = state
        robots = list(robots)
        material = list(material)
        for _ in range(minutes_left):
            new_bot = [all(r <= m for r, m in zip(rec, material)) for rec in bp]
            for i, nb, r in zip(range(4), new_bot, robots):
                material[i] += r
                #if i==0 and nb: material[i] -= rec[i]
            for i in range(4):
                if new_bot[i]:
                    robots[i] += 1
        return material[-1]

    def lower_bound(state):
        rec = bp.geode
        (minutes_left, robots, material) = state
        robots = list(robots)
        material = list(material)
        for _ in range(minutes_left):
            new_bot = all(m >= r for m, r in zip(material, rec))
            for i in range(4):
                material[i] += robots[i]
            if new_bot:
                material = [m-r for m, r in zip(material, rec)]
                robots[-1] += 1
        return material[-1]

    best = 0
    fringe = [(-1, (minutes_left, robots, material))]
    while fringe:
        upper, state = heappop(fringe)
        if -upper <= best:
            break
        for nxt in build(state):
            MAX[0] += 1
            nxt_upper = upper_bound(nxt)
            if nxt_upper > best:
                best = max(best, lower_bound(nxt))
                if nxt[0] >= 0:
                    heappush(fringe, (-nxt_upper, nxt))
    print(MAX)
    return best

class State:
    def __init__(self, bp: Materials,  minutes_left: int, robots: None | Materials, material: None | Materials):
        self.bp = bp
        self.robots = (1, 0, 0, 0) if not robots else robots
        self.material = (0, 0, 0, 0) if not material else material
        self.minutes_left = minutes_left
        self.upper: int | None = None
        self.lower: int | None = None
        self.max_cost = Materials(*(max(v) for v in zip(*self.bp)))._asdict()
        self.recipes = self.bp._asdict()

    def build(self):
        max_cost, recipes = self.max_cost, self.recipes
        (minutes_left, robots, material) = self.minutes_left, self.robots, self.material
        for purchase in ['geode', 'obsidian', 'clay', 'ore']:
            recipe = recipes[purchase]
            robot_count = getattr(robots, purchase)
            if purchase != 'geode' and robot_count >= max_cost[purchase]:
                continue
            if not all(robot_present for robot_present, ingredent_needed in zip(robots, recipe) if ingredent_needed):
                continue
            time_to_afford = max(ceil((i-m) / r) for r, m, i in zip(robots, material, recipe) if i)
            wait_time = max(0, time_to_afford) + 1
            if wait_time > minutes_left: continue
            nxt_minutes_left = minutes_left - wait_time
            nxt_material = Materials(*((m + r*(wait_time) - i) for r, m, i in zip(robots, material, recipe)))
            nxt_robots = robots._replace(**{purchase: robot_count + 1})
            # assert all(i >= 0 for i in nxt_material), (nxt_material, purchase, recipe, material, time_to_afford)
            # assert all(i >= 0 for i in nxt_robots), (nxt_material, purchase, recipe, material, time_to_afford)
            # assert nxt_minutes_left >= 0, (nxt_material, purchase, recipe, material, time_to_afford)
            yield State(self.bp, nxt_minutes_left, nxt_robots, nxt_material)
    
    def lower_bound(self):
        if self.lower:
            return self.lower
        rec = self.bp.geode
        (minutes_left, robots, material) = self.minutes_left, self.robots, self.material
        robots = list(robots)
        material = list(material)
        for _ in range(minutes_left):
            new_bot = all(m >= r for m, r in zip(material, rec))
            for i in range(4):
                material[i] += robots[i]
            if new_bot:
                material = [m-r for m, r in zip(material, rec)]
                robots[0] += 1
        
        self.lower = material[0]
        return self.lower

    def upper_bound(self):
        if self.upper:
            return self.upper
        (minutes_left, robots, material) = self.minutes_left, self.robots, self.material
        robots = list(robots)
        material = list(material)
        for _ in range(minutes_left):
            new_bot = [all(r <= m for r, m in zip(rec, material)) for rec in self.bp]
            for i, r in enumerate(robots):
                material[i] += r
            for i, build in enumerate(new_bot):
                if build:
                    robots[i] += 1
        self.upper = material[0]
        return self.upper
    
    def __lt__(self, o):
        return self.upper_bound() < o.upper_bound()
    
    def __repr__(self):
        return f'State({self.minutes_left}: rob={self.robots}, mat={self.material}) ({self.lower_bound()}-{self.upper_bound()})'

def max_geode(minutes_left: int, robots: Materials, material: Materials, bp: Materials):
    best = 0
    fringe = [(-1, State(bp, minutes_left, robots, material))]
    while fringe:
        upper, state = heappop(fringe)
        if -upper <= best:
            break
        for nxt in state.build():
            MAX[0] += 1
            nxt_upper = nxt.upper_bound()
            if nxt_upper > best:
                best = max(best, nxt.lower_bound())
                if nxt.minutes_left >= 0:
                    heappush(fringe, (-nxt_upper, nxt))
    #print(MAX, '...')
    return best

def part1(blueprints):
    robots = Materials(ore = 1)
    return sum(key * max_geode(24, robots, Materials(), bp) for key, bp in blueprints)

def part2(blueprints):
    robots = Materials(ore = 1)
    return prod(max_geode(32, robots, Materials(), bp) for _, bp in blueprints[:3])

example = '''Blueprint 1: Each ore robot costs 4 ore.  Each clay robot costs 2 ore.  Each obsidian robot costs 3 ore and 14 clay.  Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
'''
text = example
parsed = parse(text)

text = open(sys.argv[-1], 'r').read()
parsed = parse(text)
print(p1:=part1(parsed))
print(p2:=part2(parsed))
assert p1 == 1719, p1
assert p2 == 19530, p2