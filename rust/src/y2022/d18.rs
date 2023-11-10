use std::cell::RefCell;
use std::fs;
use std::rc::{Rc, Weak};

use hashbrown::{HashMap, HashSet};

type Delta = i8;
type Cube = [Delta; 3];
pub type ParsedData = HashSet<Cube>;

#[inline]
pub fn parse_str(contents: &str) -> ParsedData {
    let mut res = HashSet::with_capacity(2832);
    for row in contents.trim().split('\n') {
        let mut tmp = [0; 3];
        for (i, part) in row.split(',').enumerate() {
            tmp[i] = part.parse::<Delta>().unwrap();
        }
        res.insert(tmp);
    }
    res
}

#[inline]
pub fn parse(path: &String) -> ParsedData {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file.");
    parse_str(&contents)
}

const EYE: [[Delta; 3]; 3] = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];

#[inline]
fn neighbor_cubes(cube: Cube) -> impl Iterator<Item = Cube> {
    let [x, y, z] = cube;
    EYE.iter()
        .map(move |[dx, dy, dz]| [x + dx, y + dy, z + dz])
        .chain(EYE.iter().map(move |[dx, dy, dz]| [x - dx, y - dy, z - dz]))
}

pub fn part1(cubes: &ParsedData) -> usize {
    cubes.len() * 6
        - cubes
            .iter()
            .map(|&cube| {
                neighbor_cubes(cube)
                    .filter(|other| cubes.contains(other))
                    .count()
            })
            .sum::<usize>()
}

// Approach 0: fill the bounding box of all cubes, fastest so far!
// y2022 d18 part2 full    time:   [1.0366 ms 1.0388 ms 1.0412 ms]
pub fn a_part2(cubes: &ParsedData) -> usize {
    let cube = cubes.iter().next().unwrap();
    let mut mins = cube.clone();
    let mut maxs = cube.clone();
    for &cube in cubes {
        for d in 0..3 {
            mins[d] = mins[d].min(cube[d] - 1);
            maxs[d] = maxs[d].max(cube[d] + 1);
        }
    }
    let mut external = 0;
    let mut seen = HashSet::with_capacity(10_000);
    let mut fringe = Vec::with_capacity(5_000);
    fringe.push(mins);
    while let Some(cube) = fringe.pop() {
        for other in neighbor_cubes(cube) {
            if !(0..3).all(|i| mins[i] <= other[i] && other[i] <= maxs[i]) {
                continue;
            }
            if cubes.contains(&other) {
                external += 1;
            } else if seen.insert(other) {
                fringe.push(other);
            }
        }
    }
    external
}

// Approach 1: union-find over the faces
// Impl UnionFind over an hashmap, compact definiton but indirection is the slowest.
pub struct UnionFind {
    nodes: HashMap<Cube, (Cube, usize)>,
}

impl UnionFind {
    pub fn init() -> UnionFind {
        UnionFind {
            nodes: HashMap::with_capacity(500),
        }
    }

    pub fn insert(&mut self, cube: Cube) {
        self.nodes.entry(cube).or_insert((cube, 1));
    }

    pub fn find(&mut self, cube: &Cube) -> Cube {
        let mut root = *cube;
        while let Some(&(parent, _size)) = self.nodes.get(&root) {
            if parent == root {
                break;
            }
            root = parent;
        }
        let mut x = *cube;
        while let Some(e) = self.nodes.get_mut(&x) {
            if e.0 == root {
                break;
            }
            (x, e.0) = (e.0, root)
        }
        root
    }

    pub fn union(&mut self, x: &Cube, y: &Cube) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return;
        }

        let [(_, (p1, s1)), (_, (p2, s2))] = self.nodes.get_many_key_value_mut([&x, &y]).unwrap();

        if s1 < s2 {
            *p2 = *p1;
            *s2 += *s1;
        } else {
            *p1 = *p2;
            *s1 += *s2;
        };
    }
}

//  Other Impl of UnionFind over nodes, a bit less of indirection, faster than previous UnionFind but not as bounding box filling
struct Node {
    point: Cube,
    parent: Weak<RefCell<Node>>,
    size: usize,
}

type RNode = Rc<RefCell<Node>>;
impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.point == other.point && self.size == other.size && self.parent.ptr_eq(&other.parent)
    }
}
impl Node {
    fn new(cube: Cube) -> RNode {
        let res = Node {
            point: cube,
            parent: Weak::new(),
            size: 1,
        };
        let myself = Rc::new(RefCell::new(res));
        let binding = Rc::clone(&myself);
        {
            let mut node = (*binding).borrow_mut();
            node.parent = Rc::downgrade(&myself)
        }
        myself
    }
}

struct UnionFind2 {
    nodes: HashMap<Cube, RNode>,
}

impl UnionFind2 {
    fn init() -> UnionFind2 {
        UnionFind2 {
            nodes: HashMap::with_capacity(500),
        }
    }
    fn insert(&mut self, cube: Cube) {
        self.nodes.entry(cube).or_insert(Node::new(cube));
    }

    fn find_internal(&mut self, node: &RNode) -> RNode {
        let mut root = node.borrow().parent.upgrade().unwrap();
        while *(root.borrow().parent.upgrade().unwrap().borrow()) != *(root.borrow()) {
            let binding = Rc::clone(&root.borrow().parent.upgrade().unwrap());
            root = binding;
        }

        let mut x = Rc::clone(node);
        let wroot = Rc::downgrade(&root);
        while !(x.borrow().parent.ptr_eq(&wroot)) {
            let binding = Rc::clone(&x);
            let parent = Rc::clone(&binding.borrow().parent.upgrade().unwrap());
            let mut rebinding = binding.as_ref().borrow_mut();
            rebinding.parent = Rc::downgrade(&root);
            x = parent;
        }

        root
    }

    fn find(&mut self, cube: &Cube) -> Cube {
        let node = Rc::clone(self.nodes.get(cube).unwrap());
        self.find_internal(&node).as_ref().borrow().point
    }

    fn union_internal(&mut self, x: &RNode, y: &RNode) {
        let mut x = self.find_internal(x);
        let mut y = self.find_internal(y);
        if Rc::as_ptr(&x) == Rc::as_ptr(&y) {
            return;
        }
        if x.borrow().size < y.borrow().size {
            (x, y) = (y, x);
        }
        y.as_ref().borrow_mut().parent = Rc::downgrade(&x);
        x.as_ref().borrow_mut().size += y.borrow().size;
    }

    fn union(&mut self, x: &Cube, y: &Cube) {
        let x = Rc::clone(self.nodes.get(x).unwrap());
        let y = Rc::clone(self.nodes.get(y).unwrap());
        self.union_internal(&x, &y);
    }
}

type Face = (Cube, Cube);

#[inline]
fn faces_from_cube(cube: Cube) -> impl Iterator<Item = Face> {
    let [x, y, z] = cube;
    let other = [x + 1, y + 1, z + 1];
    EYE.iter()
        .map(move |&[dx, dy, dz]| (cube, [x + 1 - dx, y + 1 - dy, z + 1 - dz]))
        .chain(
            EYE.iter()
                .map(move |[dx, dy, dz]| ([x + dx, y + dy, z + dz], other)),
        )
}

// With UnionFind
// y2022 d18 part2 full    time:   [2.7765 ms 2.7809 ms 2.7857 ms]

// With UnionFind2
// y2022 d18 part2 full    time:   [2.1227 ms 2.1258 ms 2.1292 ms]
pub fn b_part2(cubes: &ParsedData) -> usize {
    let mut fc_cnt = HashMap::new();
    for &cube in cubes {
        for face in faces_from_cube(cube) {
            let e = fc_cnt.entry(face).or_insert(0);
            *e += 1;
        }
    }
    let faces: HashSet<Face> = fc_cnt
        .iter()
        .filter(|&(_, v)| *v == 1)
        .map(|(k, _)| *k)
        .collect();
    let mut external_cubes = HashSet::new();
    let mut union_find = UnionFind2::init();
    for &cube in cubes {
        for other in neighbor_cubes(cube) {
            if !cubes.contains(&other) {
                if external_cubes.insert(other) {
                    union_find.insert(other);
                }
            }
        }
    }
    for ecube in external_cubes.clone() {
        for other in neighbor_cubes(ecube) {
            if cubes.contains(&other) {
                continue;
            }
            if external_cubes.insert(other) {
                union_find.insert(other);
            }
            union_find.union(&ecube, &other);
        }
    }

    let mut faces_by_ecid = HashMap::new();
    for ecube in external_cubes {
        let root = union_find.find(&ecube);
        for face in faces_from_cube(ecube) {
            if faces.contains(&face) {
                let e = faces_by_ecid.entry(root).or_insert(0usize);
                *e += 1;
            }
        }
    }
    faces_by_ecid.iter().map(|(_, v)| *v).max().unwrap()
}

pub fn part2(cubes: &ParsedData) -> usize {
    a_part2(cubes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let str_input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";
        assert_eq!(part1(&parse_str(str_input)), 64);
        assert_eq!(part2(&parse_str(str_input)), 58);
    }

    #[test]
    fn actual() {
        let path = &"../inputs/2022/18.input".to_string();
        let input = &parse(path);
        assert_eq!(part1(&input), 4302);
        assert_eq!(part2(&input), 2492);
    }
}
