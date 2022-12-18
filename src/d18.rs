use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
    hash::Hash,
    io::{BufRead, Lines},
};

use crate::common;

#[derive(Eq, Clone)]
struct Coord3D {
    x: isize,
    y: isize,
    z: isize,
}

impl Coord3D {
    const fn get_bounds(&self) -> [Coord3D; 6] {
        [
            Coord3D {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            Coord3D {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            Coord3D {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            Coord3D {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            Coord3D {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
            Coord3D {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
        ]
    }
}

impl Hash for Coord3D {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl PartialEq for Coord3D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl PartialOrd for Coord3D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let xcmp = self.x.cmp(&other.x);
        match xcmp {
            Ordering::Equal => {
                let ycmp = self.y.cmp(&other.y);
                match ycmp {
                    Ordering::Equal => Some(self.z.cmp(&other.z)),
                    Ordering::Greater | Ordering::Less => Some(ycmp),
                }
            }
            Ordering::Greater | Ordering::Less => Some(xcmp),
        }
    }
}

pub fn run(input: Lines<impl BufRead>) {
    let lines = common::parse(input);

    let coord_set = process(&lines);

    let surface = compute(&coord_set);
    let exterior_surface = compute_exterior(&coord_set, surface);

    println!("surface: {}", surface);
    println!("exterior surface: {}", exterior_surface);
}

fn compute_exterior(coord_set: &HashSet<Coord3D>, initial_surface: usize) -> usize {
    let mut molding = HashSet::<Coord3D>::new();
    molding.extend(coord_set.iter().cloned());

    let mold_dim = coord_set
        .iter()
        .fold(
            [
                [isize::MAX, isize::MIN],
                [isize::MAX, isize::MIN],
                [isize::MAX, isize::MIN],
            ],
            |res, c| {
                [
                    [res[0][0].min(c.x), res[0][1].max(c.x)],
                    [res[1][0].min(c.y), res[1][1].max(c.y)],
                    [res[2][0].min(c.z), res[2][1].max(c.z)],
                ]
            },
        )
        .map(|dim| {
            // Add some space for air gap between mold wall and inner content.
            // Air gap size is 1x1x1
            [dim[0] - 2, dim[1] + 2]
        });

    // x-y space
    for mx in mold_dim[0][0]..=mold_dim[0][1] {
        for my in mold_dim[1][0]..=mold_dim[1][1] {
            let mz = mold_dim[2][0];
            molding.insert(Coord3D {
                x: mx,
                y: my,
                z: mz,
            });
            let mz = mold_dim[2][1];
            molding.insert(Coord3D {
                x: mx,
                y: my,
                z: mz,
            });
        }
    }
    // y-z space
    for my in mold_dim[1][0]..=mold_dim[1][1] {
        for mz in mold_dim[2][0]..=mold_dim[2][1] {
            let mx = mold_dim[0][0];
            molding.insert(Coord3D {
                x: mx,
                y: my,
                z: mz,
            });
            let mx = mold_dim[0][1];
            molding.insert(Coord3D {
                x: mx,
                y: my,
                z: mz,
            });
        }
    }
    // x-z space
    for mx in mold_dim[0][0]..=mold_dim[0][1] {
        for mz in mold_dim[2][0]..=mold_dim[2][1] {
            let my = mold_dim[1][0];
            molding.insert(Coord3D {
                x: mx,
                y: my,
                z: mz,
            });
            let my = mold_dim[1][1];
            molding.insert(Coord3D {
                x: mx,
                y: my,
                z: mz,
            });
        }
    }

    let mut vq = VecDeque::<Coord3D>::new();
    // Start from air gap from first corner
    vq.push_front(Coord3D {
        x: mold_dim[0][0] + 1,
        y: mold_dim[1][0] + 1,
        z: mold_dim[2][0] + 1,
    });

    while let Some(coord) = vq.pop_back() {
        molding.insert(coord.clone()).then(|| {
            for bound in coord.get_bounds().iter() {
                let need_to_check = !molding.contains(&bound);
                if need_to_check {
                    vq.push_front(bound.clone());
                }
            }
        });
    }

    let mold_surface_only = (
        // xy space
        2 * ((mold_dim[0][1] - mold_dim[0][0] + 1) * (mold_dim[1][1] - mold_dim[1][0] + 1))
    ) + (
        // xz space
        2 * ((mold_dim[0][1] - mold_dim[0][0] + 1) * (mold_dim[2][1] - mold_dim[2][0] + 1))
    ) + (
        // yz space
        2 * ((mold_dim[1][1] - mold_dim[1][0] + 1) * (mold_dim[2][1] - mold_dim[2][0] + 1))
    );

    let mold_with_air_pocket_surface = compute(&molding) as isize;

    let air_pocket_surface = mold_with_air_pocket_surface - mold_surface_only;

    let exterior_surface = initial_surface as isize - air_pocket_surface;

    exterior_surface.checked_abs().unwrap() as usize
}

fn compute(coord_set: &HashSet<Coord3D>) -> usize {
    let mut surface = 0;
    for coord in coord_set {
        let bounds = coord.get_bounds();
        for side in bounds.iter() {
            if !coord_set.contains(side) {
                surface += 1;
            }
        }
    }

    surface
}

fn process(lines: &[String]) -> HashSet<Coord3D> {
    let mut v = HashSet::<Coord3D>::new();
    for line in lines {
        let mut splt = line.split(",").map(|s| s.trim().parse::<isize>().unwrap());
        let p = Coord3D {
            x: splt.next().unwrap(),
            y: splt.next().unwrap(),
            z: splt.next().unwrap(),
        };
        v.insert(p);
    }

    v
}
