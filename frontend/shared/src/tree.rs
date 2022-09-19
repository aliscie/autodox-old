use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Tree<ID, T>
where
    ID: Hash + PartialEq + Eq + Clone + Default + Debug,
    T: PartialEq + Eq + Clone + Default + Debug,
{
    pub vertices: HashMap<ID, T>,
    pub adjacency: HashMap<ID, HashSet<ID>>,
}

impl<ID, T> Tree<ID, T>
where
    ID: Hash + PartialEq + Eq + Serialize + Clone + Default + Debug,
    T: PartialEq + Eq + Serialize + Clone + Default + Debug,
{
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            adjacency: HashMap::new(),
        }
    }
    pub fn push_vertex(&mut self, id: ID, vertex: T) {
        self.vertices.insert(id, vertex);
    }
    pub fn push_edge(&mut self, from: ID, to: ID) {
        let adjacency_to_from = self.adjacency.entry(from).or_default();
        adjacency_to_from.insert(to);
    }

    pub fn remove(&mut self, id: &ID) {
        self.vertices.remove(id);
        self.adjacency.remove(id);
        for (_id, children) in self.adjacency.iter_mut() {
            children.remove(id);
        }
    }

    pub fn into_iter<'a>(&'a self, start : ID)-> TreeIter<'a, ID, T>{
        let first_children = self.adjacency.get(&start).unwrap();
        TreeIter{
            tree : self,
            visited_nodes : HashSet::from([start]),
            stack : first_children.clone().into_iter().map(|s| (s, 0)).collect(),
            depth : 0,
        }
    }
}

//impl<'a, ID, T> IntoIterator for &'a Tree<ID, T> 
//where
    //ID: Hash + PartialEq + Eq + Clone + Default + Debug,
    //T: PartialEq + Eq + Clone + Default + Debug,
//{
    //type Item = &'a T;
    //type IntoIter = TreeIter<'a, ID, T>;
    //fn into_iter(self) -> Self::IntoIter {
        //TreeIter {
            //tree: self,
            //visited_nodes: HashSet::new(),
            //stack: HashSet::new(),
        //}
    //}
//}

pub struct TreeIter<'a, ID, T>
where
    ID: Hash + PartialEq + Eq + Clone + Default + Debug,
    T: PartialEq + Eq + Clone + Default + Debug,
{
    tree: &'a Tree<ID, T>,
    visited_nodes: HashSet<ID>,
    //stack: HashSet<ID>,
    stack : VecDeque<(ID, u64)>,
    depth : u64,
}

impl<'a, ID, T> Iterator for TreeIter<'a, ID, T>
where
    ID: Hash + PartialEq + Eq + Clone + Default + Debug,
    T: PartialEq + Eq + Clone + Default + Debug,
{
    type Item = (&'a T, u64);
    fn next(&mut self) -> Option<Self::Item> {
        while self.stack.len() > 0{
            let data = self.stack.pop_front().unwrap();
            if self.visited_nodes.contains(&data.0){
                // that node has already been visited go ahead
                continue;
            }
            let node = self.tree.vertices.get(&data.0).unwrap();
            if let Some(children) = self.tree.adjacency.get(&data.0){
                for i in children{
                    if !self.visited_nodes.contains(i){
                        self.stack.push_front((i.clone(), self.depth));
                    }
                }
            }
            self.visited_nodes.insert(data.0);
            return Some((node, data.1));
        }
        None
    }
}
