//Tam, kad paleisti programą, į aplanką kuriame yra aplankas src reikia
//įkelti failą 2743_1234.las

mod a_star;
mod model;

use las::{Bounds, Read, Reader, Point};
use std::collections::LinkedList;

use crate::{model::Octree, a_star::{State, Problem}};

fn main() {
    println!("Reading file...");

    let init_bounds = get_init_bounds(Reader::from_path("2743_1234.las").unwrap());

    println!("bounds: {:?}", init_bounds);

    println!("Finished");

    println!("subdividing...");

    let mut octree = Octree::new(init_bounds, 0);

    let mut point_a = None;
    let mut point_b = None;

    let mut iterations = 0;

    for wrapped_point in Reader::from_path("2743_1234.las").unwrap().points() {
        let point = wrapped_point.unwrap();
        if iterations == 15{
            point_a = Some(point.clone());
        }
        else {
            point_b = Some(point.clone());
        }
        octree.insert_point(point, 5);
        iterations += 1;
    }

    println!("Finished");

    println!("Searching");

    //println!("point_a: {:?}, point_b: {:?}", point_a, point_b);
    
    let initial_state = State::new(point_a.unwrap(), &octree);
    let goal_state = State::new(point_b.unwrap(), &octree);

    //println!("initial_state: {:?}, goal_state: {:?}", initial_state, goal_state);

    if let Some(initial_state) = initial_state {
        if let Some(goal_state) = goal_state {
            let mut prob = Problem::new(initial_state, goal_state);
            println!("Path: {:?}", prob.search());
        }
    }
    




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
