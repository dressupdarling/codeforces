use std::fs;
use std::collections::BinaryHeap;

type Coord = i64;

#[derive(Clone, Copy)]
struct Point {
    x: Coord,
    y: Coord,
    z: Coord,
}

impl Point {
    fn distance_sq(&self, other: &Point) -> Coord {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let mut root = x;
        while self.parent[root] != root {
            self.parent[root] = self.parent[self.parent[root]];
            root = self.parent[root];
        }
        root
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let mut root_x = self.find(x);
        let mut root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.size[root_x] < self.size[root_y] {
            std::mem::swap(&mut root_x, &mut root_y);
        }

        self.parent[root_y] = root_x;
        self.size[root_x] += self.size[root_y];
        self.components -= 1;
        true
    }

    fn component_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        let mut sizes = Vec::new();
        for i in 0..n {
            if self.parent[i] == i {
                sizes.push(self.size[i]);
            }
        }
        sizes
    }
}

fn main() {
    let input = fs::read_to_string("day8_input.txt").unwrap();
    let points: Vec<Point> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let mut parts = line.split(',');
            Point {
                x: parts.next().unwrap().trim().parse().unwrap(),
                y: parts.next().unwrap().trim().parse().unwrap(),
                z: parts.next().unwrap().trim().parse().unwrap(),
            }
        })
        .collect();

    let n = points.len();
    let grid_size = 10;

    // Find bounds
    let (min_x, max_x, min_y, max_y, min_z, max_z) = points.iter().fold(
        (Coord::MAX, Coord::MIN, Coord::MAX, Coord::MIN, Coord::MAX, Coord::MIN),
        |(min_x, max_x, min_y, max_y, min_z, max_z), p| {
            (
                min_x.min(p.x),
                max_x.max(p.x),
                min_y.min(p.y),
                max_y.max(p.y),
                min_z.min(p.z),
                max_z.max(p.z),
            )
        },
    );

    let range_x = (max_x - min_x).max(1);
    let range_y = (max_y - min_y).max(1);
    let range_z = (max_z - min_z).max(1);
    let mut grid: Vec<Vec<usize>> = vec![Vec::new(); grid_size * grid_size * grid_size];
    for (idx, point) in points.iter().enumerate() {
        let gx = ((point.x - min_x) * grid_size as Coord / range_x) as usize;
        let gy = ((point.y - min_y) * grid_size as Coord / range_y) as usize;
        let gz = ((point.z - min_z) * grid_size as Coord / range_z) as usize;

        let gx = gx.min(grid_size - 1);
        let gy = gy.min(grid_size - 1);
        let gz = gz.min(grid_size - 1);

        grid[(gx * grid_size + gy) * grid_size + gz].push(idx);
    }

    let mut heap = BinaryHeap::new();
    let total_cells = grid_size * grid_size * grid_size;

    for cell_idx in 0..total_cells {
        let cell_x = cell_idx / (grid_size * grid_size);
        let cell_y = (cell_idx / grid_size) % grid_size;
        let cell_z = cell_idx % grid_size;

        for dx in 0..2 {
            for dy in -1..2 {
                for dz in -1..2 {
                    let nx = cell_x as isize + dx;
                    let ny = cell_y as isize + dy;
                    let nz = cell_z as isize + dz;

                    if nx < 0 || nx >= grid_size as isize ||
                        ny < 0 || ny >= grid_size as isize ||
                        nz < 0 || nz >= grid_size as isize {
                        continue;
                    }

                    let neighbor_idx = (nx as usize) * grid_size * grid_size +
                        (ny as usize) * grid_size +
                        (nz as usize);

                    if neighbor_idx < cell_idx && (dx, dy, dz) != (0, 0, 0) {
                        continue;
                    }

                    let current_points = &grid[cell_idx];
                    let neighbor_points = &grid[neighbor_idx];

                    for &i in current_points {
                        for &j in neighbor_points {
                            if neighbor_idx == cell_idx && i >= j {
                                continue;
                            }

                            let dist = points[i].distance_sq(&points[j]);
                            heap.push((dist, i, j));
                        }
                    }
                }
            }
        }
    }

    let mut edges: Vec<(Coord, usize, usize)> = Vec::with_capacity(heap.len());
    while let Some((d, i, j)) = heap.pop() {
        edges.push((d, i, j));
    }
    edges.sort_unstable_by_key(|&(d, _, _)| d);

//Part 1

    let k = 1000;
    let mut uf1 = UnionFind::new(n);
    for (_, i, j) in edges.iter().take(k) {
        uf1.union(*i, *j);
    }

    let mut sizes = uf1.component_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    let part1 = if sizes.len() >= 3 {
        sizes[0] * sizes[1] * sizes[2]
    } else {
        0
    };

    println!("Part 1: {}", part1);

//Part 2

    let mut uf2 = UnionFind::new(n);
    let mut last_x_product = 0;

    for (_, i, j) in edges.iter() {
        if uf2.union(*i, *j) {
            if uf2.components == 1 {
                last_x_product = points[*i].x * points[*j].x;
                break;
            }
        }
    }

    println!("Part 2: {}", last_x_product);
}