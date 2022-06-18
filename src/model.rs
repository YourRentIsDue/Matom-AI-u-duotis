use std::collections::LinkedList;

use std::hash::Hash;

use las::{Bounds, Point, Vector};
#[derive(Clone, Debug)]
pub struct Octree {
    pub depth: i32,
    pub octants: Option<[Bounds; 8]>,
    pub children: [Option<Box<Octree>>; 8],
    pub points: Vec<Point>,
    pub bounds: Bounds,
}
#[derive(Debug)]
enum SearchError{
    NotFound
}

impl Hash for Octree {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.depth.hash(state);
        self.children.hash(state);
    }
}

impl PartialEq for Octree {
    fn eq(&self, other: &Self) -> bool {
        self.depth == other.depth && self.octants == other.octants && self.children == other.children && self.points == other.points && self.bounds == other.bounds
    }
}

impl Eq for Octree {
    
}

pub trait Comparison {
    fn compare_area(&self, compare_to: Bounds) -> bool;
    fn contains_point(&self, point: &Point) -> bool;
    fn contains_area(&self, area: Bounds) -> bool;
    fn overlaps_area(&self, area: Bounds) -> bool;
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
    fn contains_point(&self, point: &Point) -> bool {
        if self.min.x <= point.x
            && self.max.x >= point.x
            && self.min.y <= point.y
            && self.max.y >= point.y
            && self.min.z <= point.z
            && self.max.z >= point.z
        {
            true
        } else {
            false
        }
    }

    fn contains_area(&self, area: Bounds) -> bool {
        if self.min.x <= area.min.x
            && self.max.x >= area.max.x
            && self.min.y <= area.min.y
            && self.max.y >= area.max.y
            && self.min.z <= area.min.z
            && self.max.z >= area.max.z
        {
            true
        } else {
            false
        }
    }

    fn overlaps_area(&self, area: Bounds) -> bool {
        if (self.min.x >= area.min.x && self.min.x <= area.max.x)
            || (self.max.x >= area.min.x && self.max.x <= area.max.x)
            || (self.min.y >= area.min.y && self.min.y <= area.max.y)
            || (self.max.y >= area.min.y && self.max.y <= area.max.y)
            || (self.min.z >= area.min.z && self.min.z <= area.max.z)
            || (self.max.z >= area.min.z && self.max.z <= area.max.z)
        {
            true
        } else {
            false
        }
    }
}

impl Octree {
    pub fn new(bounds: Bounds, depth: i32) -> Self {
        let half_of_x = bounds.min.x + ((bounds.max.x - bounds.min.x) / 2.0);
        let half_of_y = bounds.min.y + ((bounds.max.y - bounds.min.y) / 2.0);
        let half_of_z = bounds.min.z + ((bounds.max.z - bounds.min.z) / 2.0);

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
            bounds: bounds,
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

    pub fn get_all_points(&self) -> Vec<&Point> {
        let mut output = Vec::new();

        for point in &self.points {
            output.push(point);
        }

        for child in &self.children {
            if let Some(child) = child {
                for point in child.get_all_points() {
                    output.push(point);
                }
            }
        }
        output
    }

    pub fn insert_point(&mut self, point: Point, max_depth: i32) {
        for i in 0..8 {
            if self.octants.unwrap()[i].contains_point(&point) {
                if self.depth + 1 < max_depth {
                    if let None = self.children[i] {
                        self.children[i] = Some(Box::new(Octree::new(
                            self.octants.unwrap()[i],
                            self.depth + 1,
                        )));
                    }
                    self.children[i]
                        .as_mut()
                        .unwrap()
                        .insert_point(point, max_depth);
                    return;
                }
            }
        }
        self.points.push(point);
    }

    pub fn search_for_octant(&self, query: &Point) -> Option<Box<Octree>> {
       
        


        if self.points.contains(query) {
            println!("runs");
            return Some(Box::new(self.clone()));
        }
        else {
            for child in &self.children {
                if let Some(child) = child {
                    if let Some(result) = child.search_for_octant(query) {
                        return Some(result);
                    }
                }
            }
            return None
        }


        
    }
        
        
    

    pub fn equals(&self, comparing_to: &Box<Octree>) -> bool {
        if self.depth == comparing_to.depth && self.bounds == comparing_to.bounds {
            true
        } else {
            false
        }
    }

    pub fn find_parent(&self, of: &Octree) -> Option<Box<Octree>> {
        if self.bounds.contains_area(of.bounds) && self.depth == of.depth - 1 {
            return Some(Box::new(self.clone()));
        } else {
            for child in &self.children {
                if let Some(child) = child {
                    if let Some(parent) = child.find_parent(of){
                        return Some(parent);
                    }
                }
            }
        }
        None
    }

    pub fn search(&mut self, query: Bounds, list: &mut LinkedList<Point>) {
        for point in &self.points {
            if query.contains_point(point) {
                list.push_back(point.clone());
            }
        }
        for child in &mut self.children {
            match child {
                Some(child) => {
                    if query.contains_area(child.bounds) {
                        for point in child.get_all_points() {
                            list.push_back(point.clone());
                        }
                    } else if child.bounds.overlaps_area(query) {
                        child.search(query, list);
                    }
                }
                None => {}
            }
        }
    }
}
