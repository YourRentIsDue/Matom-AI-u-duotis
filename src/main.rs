//Tam, kad paleisti programą, į aplanką kuriame yra aplankas src reikia 
//įkelti failą 2743_1234.las

use las::{Bounds, Point, Read, Reader, Vector};

struct Octree {
    depth: i32,
    octants: Option<[Bounds; 8]>,
    children: [Option<Box<Octree>>; 8],
    points: Vec<Point>,
}

trait Comparison {
    fn compare_area(&self, compare_to: Bounds) -> bool;
}

impl Comparison for Bounds {
    fn compare_area(&self, compare_to: Bounds) -> bool {
        let self_area =
            (self.max.x - self.min.x) * (self.max.y - self.min.y) * (self.max.z - self.min.z);
        let comparing_to_area = (compare_to.max.x - compare_to.min.x)
            * (compare_to.max.y - compare_to.min.y)
            * (compare_to.max.z - compare_to.min.z);


        if self_area > comparing_to_area {
            true
        } else {
            false
        }
    }
}

impl Octree {
    pub fn new(bounds: Bounds, depth: i32) -> Self {
        let half_of_x = bounds.max.x / 2.0;
        let half_of_y = bounds.max.y / 2.0;
        let half_of_z = bounds.max.z / 2.0;

        let octants = [
            Bounds {
                //south west
                min: bounds.min,
                max: Vector {
                    x: half_of_x,
                    y: half_of_y,
                    z: half_of_z,
                },
            },
            Bounds {
                //south east
                min: Vector {
                    x: half_of_x,
                    y: bounds.min.y,
                    z: bounds.min.z,
                },
                max: Vector {
                    x: bounds.max.x,
                    y: half_of_y,
                    z: half_of_z,
                },
            },
            Bounds {
                //north west
                min: Vector {
                    x: bounds.min.x,
                    y: half_of_y,
                    z: bounds.min.z,
                },
                max: Vector {
                    x: half_of_x,
                    y: bounds.max.y,
                    z: half_of_z,
                },
            },
            Bounds {
                //north east
                min: Vector {
                    x: half_of_x,
                    y: half_of_y,
                    z: bounds.min.z,
                },
                max: Vector {
                    x: bounds.max.x,
                    y: bounds.max.y,
                    z: half_of_z,
                },
            },
            Bounds {
                min: Vector {
                    x: bounds.min.x,
                    y: bounds.min.y,
                    z: half_of_z,
                },
                max: Vector {
                    x: half_of_x,
                    y: half_of_y,
                    z: bounds.max.z,
                },
            },
            Bounds {
                min: Vector {
                    x: half_of_x,
                    y: bounds.min.y,
                    z: half_of_z,
                },
                max: Vector {
                    x: bounds.max.x,
                    y: half_of_y,
                    z: bounds.max.z,
                },
            },
            Bounds {
                min: Vector {
                    x: bounds.min.x,
                    y: half_of_y,
                    z: half_of_z,
                },
                max: Vector {
                    x: half_of_x,
                    y: bounds.max.y,
                    z: bounds.max.z,
                },
            },
            Bounds {
                min: Vector {
                    x: half_of_x,
                    y: half_of_y,
                    z: half_of_z,
                },
                max: bounds.max,
            },
        ];

        Octree {
            depth: depth,
            octants: Some(octants),
            children: [None, None, None, None, None, None, None, None],
            points: Vec::new(),
        }
    }

    pub fn get_point_count(&self) -> usize {
        let mut point_count = self.points.len();
        for child in &self.children {
            match child {
                Some(child) => point_count += child.get_point_count(),
                None => {}
            }
        }
        point_count
    }

    pub fn insert_point(&mut self, point: Point, max_depth: i32) {
        let point_size = Bounds {
            min: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            max: Vector {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        };

        for i in 0..8 {
            if self.octants.unwrap()[i].compare_area(point_size) {
                if self.depth + 1 < max_depth{
                    if let None = self.children[i] {
                        self.children[i] = Some(Box::new(Octree::new(
                            self.octants.unwrap()[i],
                            self.depth + 1,
                        )));
                    }
                    self.children[i].as_mut().unwrap().insert_point(point, max_depth);
                    return;
                }          
            }
        }
        self.points.push(point);
    }
}

fn main() {

    let init_bounds = get_init_bounds(Reader::from_path("2743_1234.las").unwrap());

    println!("init bounds: {:?}", init_bounds);

    let mut octree = Octree::new(init_bounds, 0);

    println!("runs");


    for wrapped_point in Reader::from_path("2743_1234.las").unwrap().points() {
        let point = wrapped_point.unwrap();
        octree.insert_point(point, 5);
    }

    println!("Points in octree: {}", octree.get_point_count());
}

fn get_init_bounds(mut reader: Reader) -> Bounds {
    let mut init_bounds = Bounds {
        ..Default::default()
    };

    for wrapped_point in reader.points() {
        let point = wrapped_point.unwrap();
        init_bounds.grow(&point)
    }

    init_bounds
}
