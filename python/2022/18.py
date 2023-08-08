import sys
from itertools import cycle
from collections import Counter, defaultdict

class Node:
    __slots__ = ['parent', 'size', 'point']
    def __init__(self, point, parent=None, size=1):
        self.point = point
        self.parent = parent if parent else self
        self.size = size
    def __repr__(self):
        return repr(dict(point=self.point, parent=self.parent.point, size=self.size))

def find(node):
    root = node
    while root.parent != root:
        root = root.parent
    x = node
    assert x != None, (node, root, x)
    while x.parent != root:
        parent = x.parent
        x.parent = root
        x = parent

    return root

def union(x, y):
    x = find(x)
    y = find(y)
    if x == y:
        return
    if x.size < y.size:
        x, y = y, x
    y.parent = x
    x.size = x.size + y.size


def faces_from_cube(cube):
    x, y, z = cube
    yield (x, y, z), (x, y+1, z+1)
    yield (x, y, z), (x+1, y, z+1)
    yield (x, y, z), (x+1, y+1, z)

    yield (x+1, y, z), (x+1, y+1, z+1)
    yield (x, y+1, z), (x+1, y+1, z+1)
    yield (x, y, z+1), (x+1, y+1, z+1)

def parse(text):
    face_c = Counter()
    cubes = set()
    for line in text.split('\n'):
        cube = tuple(map(int, line.split(',')))
        cubes.add(cube)
        for face in faces_from_cube(cube):
            face_c[face] += 1
    return set(face for face, cnt in face_c.items() if cnt == 1), cubes

def part1(data):
    faces, _ = data
    return len(faces)

def neighbor_cubes(cube):
    x, y, z = cube
    yield x+1, y, z
    yield x, y+1, z
    yield x, y, z+1
    yield x-1, y, z
    yield x, y-1, z
    yield x, y, z-1

def part2(data):
    faces, cubes = data
    external_cubes = dict()
    for cube in cubes:
        for other in neighbor_cubes(cube):
            if other not in cubes:
                if other not in external_cubes:
                    external_cubes[other] = Node(other)
    for ecube, enode in list(external_cubes.items()):
        for other in neighbor_cubes(ecube):
            if other in cubes: continue
            if not (other_node := external_cubes.get(other)):
                other_node = external_cubes[other] = Node(other)
            union(enode, other_node)
    faces_by_ecid = defaultdict(set)
    for ecube, enode in external_cubes.items():
        root = find(enode).parent
        root = root if root else enode
        for face in faces_from_cube(ecube):
            if face in faces:
                faces_by_ecid[root.point].add(face)
    return max(len(i) for i in faces_by_ecid.values())

# prev = (parse, part1, part2)
# def parse(text):
#     cubes = set()
#     for line in text.split('\n'):
#         cube = tuple(map(int, line.split(',')))
#         cubes.add(cube)
#     return cubes

# def part1(cubes):
#     return len(cubes) * 6 - sum(1 for cube in cubes for other in neighbor_cubes(cube) if other in cubes)

# def part2(cubes):
#     x, y, z = next(iter(cubes))
#     mins = x-1, y-1, z-1
#     maxs = x+1, y+1, z-1
#     for cube in cubes:
#         mins = [min(i,j-1) for i, j in zip(mins, cube)]
#         maxs = [max(i,j+1) for i, j in zip(maxs, cube)]
#     external = 0
#     seen = set()
#     fringe = [tuple(mins)]
#     while fringe:
#         cube = fringe.pop()
#         if cube in seen: continue
#         seen.add(cube)
#         for other in neighbor_cubes(cube):
#             if not all((i <= j <= k) for i,j,k in zip(mins, cube, maxs)):
#                 continue
#             if other in cubes:
#                 external += 1
#             elif other not in seen:
#                 fringe.append(other)
#     return external

# def part2(cubes):
#     x, y, z = next(iter(cubes))
#     mins = x-1, y-1, z-1
#     maxs = x+1, y+1, z-1
#     for cube in cubes:
#         mins = [min(i,j-1) for i, j in zip(mins, cube)]
#         maxs = [max(i,j+1) for i, j in zip(maxs, cube)]
#     external = 0
#     seen = set()
#     fringe = [tuple(mins)]
#     while fringe:
#         cube = fringe.pop()
#         # if cube in seen: continue
#         # seen.add(cube)
#         for other in neighbor_cubes(cube):
#             if not all((i <= j <= k) for i,j,k in zip(mins, cube, maxs)):
#                 continue
#             if (key:=(cube, other)) not in seen:
#                 seen.add(key)
#                 if other in cubes:
#                     external += 1
#                 elif other not in seen:
#                     seen.add(other)
#                     fringe.append(other)
#             # if not all((i <= j <= k) for i,j,k in zip(mins, cube, maxs)):
#             #     continue
#             # if other in cubes:
#             #     external += 1
#             # elif other not in seen:
#             #     fringe.append(other)
#     return external

# parse, part1, part2 = prev

lines = open(sys.argv[-1]).read().strip()
# _lines = '''1,1,1
# 2,1,1'''
# instrs = parse(_lines)
# p1 = part1(instrs)
# assert p1 == 10, p1
# _lines = '\n'.join('%d,%d,%d'%(x,y,z) for x in range(3) for y in range(3) for z in range(3) if (x,y,z) != (1,1,1))
# _lines = '''0,0,0
# 0,0,2
# 0,-1,1
# 0,1,1
# 1,0,1
# -1,0,1'''
# instrs = parse(_lines)
# p1 = part1(instrs)
# assert p1 == 36, p1
# p2 = part2(instrs)
# assert p2 == 30, p2

# _lines = '''2,2,2
# 1,2,2
# 3,2,2
# 2,1,2
# 2,3,2
# 2,2,1
# 2,2,3
# 2,2,4
# 2,2,6
# 1,2,5
# 3,2,5
# 2,1,5
# 2,3,5'''
# instrs = parse(_lines)
# p1 = part1(instrs)
# assert p1 == 64, p1
# p2 = part2(instrs)
# assert p2 == 58, p2

instrs = parse(lines)
p1 = part1(instrs)
#assert p1 == 4302, p1
print(p1)
p2 = part2(instrs)
#assert p2 == 2492, p2
print(p2)
