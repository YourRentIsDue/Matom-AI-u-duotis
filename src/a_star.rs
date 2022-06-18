use std::{
    collections::{HashMap, LinkedList, BTreeMap},
    ops::Index,
};

use las::Point;

use crate::model::Comparison;
use crate::model::Octree;

#[derive(Clone, Debug)]
pub struct Node {
    pub state: State,
    pub parent: Option<Box<Node>>,
    pub action: Option<Action>,
}
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct State {
    pub start: Box<Octree>,
    pub tree: Box<Octree>,
}

pub struct ActionStatePair {
    pub action: Action,
    pub state: State,
}
#[derive(Clone, Debug)]
pub struct Action {
    pub move_to: Box<Octree>,
    pub move_from: Box<Octree>,
    pub cost: i32,
}
#[derive(Debug)]
pub struct Path {
    pub total_cost: i32,
    pub nodes: Vec<Node>,
}

pub struct Problem {
    pub nodes_visited: i32,
    pub start_state: State,
    pub goal_state: State,
}

impl Node {
    pub fn get_cost(&self) -> i32 {
        let mut result = 0;

        let mut current_node = self.clone();

        while let Some(parent) = current_node.parent.clone() {
            if let Some(action) = parent.action.clone() {
                result += action.clone().cost; 
            }
            
            current_node = *parent;
        }
        result
    }
}

impl Problem {


    pub fn new(initial_state:State, goal_state:State) -> Self {
        Problem { nodes_visited: 0, start_state: initial_state, goal_state: goal_state }
    }

    pub fn is_goal(&self, state: State) -> bool {
        state.equals(self.goal_state.clone())
    }

    pub fn construct_path(mut node: Node) -> Path {
        let mut result = Path {
            total_cost: 0,
            nodes: Vec::new(),
        };

        result.total_cost += node.get_cost();
        result.nodes.push(node.clone());

        while let Some(parent) = node.parent.clone() {
            result.total_cost += parent.get_cost();
            result.nodes.push(*parent.clone());
            node = *parent.clone();
        }
        result
    }

    pub fn add_child_binary(
        &self,
        fringe: &mut Vec<Node>,
        node: Node,
        mut left: usize,
        right: &mut usize,
    ) {
        while (true) {
            if left > *right {
                fringe.insert(left.try_into().unwrap(), node);
                return;
            }
            let node_value = self.evaluation(node.clone());
            if left == *right {
                let left_value = self.evaluation(fringe[left].clone());
                if left_value > node_value {
                    fringe.insert(left, node);
                    return;
                }
                if left_value == node_value {
                    if fringe[left].get_cost() > node.get_cost() {
                        fringe.insert(left + 1, node);
                        return;
                    }
                    fringe.insert(left, node);
                    return;
                }
                fringe.insert(left + 1, node);
                return;
            }
            let mid = (left + *right) / 2;
            
            let mut mid_value = 0;

            if let Some(node) = fringe.get(mid){
                mid_value = self.evaluation(node.clone());
            }
            if mid_value == node_value {
                if fringe[mid].get_cost() > node.get_cost() {
                    fringe.insert(mid + 1, node);
                    return;
                }
                fringe.insert(mid, node);
                return;
            }
            if mid_value > node_value {
                *right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
    }

    pub fn evaluation(&self, node: Node) -> i32 {
        node.get_cost() + self.heuristic(node.state)
    }

    pub fn heuristic(&self, current_state: State) -> i32 {
        if current_state.start.depth > self.goal_state.start.depth {
            if self
                .goal_state
                .start
                .bounds
                .contains_area(current_state.start.bounds)
            {
                current_state.start.depth - self.goal_state.start.depth
            } else {
                (current_state.start.depth - self.goal_state.start.depth) + 1
            }
        } else if current_state.start.depth < self.goal_state.start.depth {
            if current_state
                .start
                .bounds
                .contains_area(self.goal_state.start.bounds)
            {
                self.goal_state.start.depth - current_state.start.depth
            } else {
                (self.goal_state.start.depth - current_state.start.depth) + 1
            }
        } else {
            match current_state.tree.find_parent(&current_state.start) {
                Some(current_state_parent) => {
                    match self.goal_state.tree.find_parent(&self.goal_state.start) {
                        Some(goal_state_parent) => {
                            if current_state_parent.bounds == goal_state_parent.bounds {
                                current_state_parent.depth - current_state.start.depth
                            } else {
                                current_state_parent.depth - current_state.start.depth + 2
                            }
                        }
                        None => 100000000,
                    }
                }
                None => 100000000,
            }
        }
    }

    pub fn search(&mut self) -> Option<Path> {
        //NOTE: originally I wanted to use a hash map for this
        //however, it does not work without trait implementations for structs in the LAS library
        //that I can not modify without modifying the library itself.
        let mut visited_nodes = HashMap::new();

        

        let mut fringe = Vec::new();

        let root_node = Node {
            state: self.start_state.clone(),
            parent: None,
            action: None,
        };
        fringe.push(root_node.clone());
        visited_nodes.insert(root_node.clone().state, root_node);
        

        self.nodes_visited += 1;

        if self.nodes_visited % 1000 == 0 {
            println!("Explored {} Nodes", self.nodes_visited);
        }

        while true {
            println!("Nodes visited: {}", self.nodes_visited);
            if fringe.is_empty() {
                
                return None;
            }
            let node = fringe.remove(0);

            println!("node depth: {}", node.state.start.depth);

            if self.is_goal(node.state.clone()) {
                println!("goal found");
                return Some(Problem::construct_path(node.clone()));
            }

            let child_nodes = node.state.successor();
            for child in child_nodes {
                self.nodes_visited += 1;
                if self.nodes_visited % 1000 == 0 {
                    println!("Explored {} Nodes", self.nodes_visited);
                }
                let last_seen_node = visited_nodes.get_mut(&child.state);

                match last_seen_node {
                    Some(last_seen_node) => {
                        if last_seen_node.get_cost()>child.action.cost + node.get_cost(){
                            last_seen_node.parent = Some(Box::new(node.clone()));
                            last_seen_node.action = Some(child.action.clone());
                        }
                    }
                    None => {
                        let child_node = Node {
                            state: child.state,
                            parent: Some(Box::new(node.clone())),
                            action: Some(child.action),
                        };

                        let mut right = fringe.len() - 1;

                        self.add_child_binary(&mut fringe, child_node, 0, &mut right);
                    },
                }
                
            }
        }
        None
    }
}

impl State {
    pub fn new(start_point: Point, tree: &Octree) -> Option<Self> {
        
        let starting_octant = tree.search_for_octant(&start_point);
        
        if let Some(starting_octant) = starting_octant {
            return Some(State {
                    start: Box::new(*starting_octant),
                    tree: Box::new(tree.clone()),
                });
        } else {
            return None
        }
    }

    pub fn successor(&self) -> Vec<ActionStatePair> {
        let mut result = Vec::new();


        
        if let Some(parent) = self.tree.find_parent(&self.start) {
            let action = Action {
                move_to: parent,
                move_from: self.start.clone(),
                cost: 1,
            };
            let next_state = State {
                start: action.move_to.clone(),
                tree: self.tree.clone(),
            };
            result.push(ActionStatePair {
                action: action,
                state: next_state,
            });
        }
        else {
            println!("None");
        }

        for child in self.start.clone().children {
            if let Some(child) = child {
                let action = Action {
                    move_to: child,
                    move_from: self.start.clone(),
                    cost: 1,
                };
                let next_state = State {
                    start: action.move_to.clone(),
                    tree: self.tree.clone(),
                };
                result.push(ActionStatePair {
                    action: action,
                    state: next_state,
                });
            }
        }

        

        result
    }

    pub fn equals(&self, state_to_check: State) -> bool {
        self.start.equals(&state_to_check.start) && self.tree.equals(&state_to_check.tree)
    }
}
