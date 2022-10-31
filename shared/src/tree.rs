use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;
use uuid::Uuid;

#[cfg(feature = "tauri")]
use surrealdb::sql::{Id, Value,Object, Array};

use crate::schema::FileNode;
use crate::traits::Entity;
use crate::Error;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Tree<ID, T>
where
    ID: Hash + PartialEq + Eq + Clone + Default + Debug,
    T: PartialEq + Eq + Clone + Debug,
{
    pub vertices: HashMap<ID, T>,
    pub adjacency: HashMap<ID, IndexSet<ID>>,
    pub root: Option<ID>,
}

impl<ID, T> Tree<ID, T>
where
    ID: Hash + PartialEq + Eq + Serialize + Clone + Default + Debug,
    T: PartialEq + Eq + Serialize + Clone + Debug,
{
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            adjacency: HashMap::new(),
            root: None,
        }
    }
    pub fn push_vertex(&mut self, id: ID, vertex: T) {
        self.vertices.insert(id, vertex);
    }
    pub fn push_edge(&mut self, from: ID, to: ID) {
        let adjacency_to_from = self.adjacency.entry(from).or_default();
        adjacency_to_from.insert(to);
    }
    pub fn delete_edge(&mut self, parent_id: ID, child_id: ID) {
        let adjacency = self.adjacency.entry(parent_id).or_default();
        adjacency.swap_remove(&child_id);
    }

    pub fn push_children(&mut self, parent_id: ID, child_id: ID, child: T) {
        self.push_vertex(child_id.clone(), child);
        self.push_edge(parent_id, child_id);
    }

    pub fn len(&self) -> usize {
        self.vertices.len()
    }
    pub fn len_from_start(&self, id: &ID) -> usize {
        let mut stack = VecDeque::from([id]);
        let mut visited_nodes = HashSet::from([id]);
        while stack.len() > 0 {
            let id = stack.pop_front().unwrap();
            if let Some(children) = self.adjacency.get(id) {
                for i in children {
                    if !visited_nodes.contains(i) {
                        stack.push_back(i);
                        visited_nodes.insert(i);
                    }
                }
            }
        }
        visited_nodes.len()
    }

    pub fn remove(&mut self, id: &ID) {
        let mut remove_stack = VecDeque::from([id.clone()]);
        while remove_stack.len() > 0 {
            let r = remove_stack.pop_front().unwrap();
            self.vertices.remove(&r);
            if let Some(children_id) = self.adjacency.remove(&r) {
                for i in children_id {
                    remove_stack.push_front(i);
                }
            }
        }
        for (_id, children) in self.adjacency.iter_mut() {
            children.remove(id);
        }
    }

    pub fn into_iter<'a>(&'a self, start: ID) -> TreeIter<'a, ID, T> {
        //let first_children = self.adjacency.get(&start).unwrap();
        let len = self.len_from_start(&start);
        let mut stack: VecDeque<ID> = VecDeque::new();
        stack.push_front(start.clone());
        let mut other_stack = VecDeque::new();
        let mut visited_nodes = HashSet::new();
        while visited_nodes.len() < len {
            let data = stack.pop_front().unwrap();
            if let Some(children) = self.adjacency.get(&data) {
                for i in children {
                    if !visited_nodes.contains(i) {
                        stack.push_back(i.clone());
                    }
                }
            }
            visited_nodes.insert(data.clone());
            other_stack.push_front(data.clone());
        }
        TreeIter {
            tree: self,
            stack: other_stack,
        }
    }
}

#[cfg(feature = "tauri")]
impl<ID, T> From<Tree<ID, T>> for Object
where
    ID: Hash + PartialEq + Eq + Clone + Default + Debug,
    T: PartialEq + Eq + Clone + Debug + Into<Object> + Entity,
{
    fn from(val: Tree<ID, T>) -> Object {
        let mut map = BTreeMap::new();
        let mut adjacency_object = BTreeMap::new();
        for (id, children) in val.adjacency {
            let mut child_connections: Vec<Value> = Vec::new();
            for i in children.into_iter() {
                println!("child_ids here is : {:?}", i);
                child_connections.push(Value::Strand(format!("{:?}", i).into()).into());
            }
            println!("{:?}", child_connections);
            adjacency_object.insert(format!("{:?}", id), Value::Array(child_connections.into()));
        }
        let mut vertices_vec: Vec<Value> = Vec::new();
        for i in val.vertices.keys() {
            vertices_vec.push(Value::Thing((T::table_name(), format!("{:?}", i)).into()));
        }
        map.insert(
            "adjacency".to_owned(),
            Value::Object(adjacency_object.into()),
        );
        map.insert("vertex".to_owned(), Value::Array(vertices_vec.into()));
        map.insert(
            "root".to_owned(),
            val.root.map(|f| format!("{:?}", f)).into(),
        );
        return map.into();
    }
}

#[cfg(feature = "tauri")]
impl TryFrom<Object> for Tree<Uuid, FileNode> {
    type Error = crate::Error;
    /// i am asuming we have selected the vertices field with all the data in the nodes
    fn try_from(value: Object) -> Result<Self, Self::Error> {
        let mut value = value;
        let mut tree = Tree::new();
        let adjacency_list: Object = value
            .remove("adjacency")
            .ok_or(crate::Error::XPropertyNotFound("adjacency".into()))?
            .try_into()
            .map_err(|_| Error::XValueNotOfType("value"))?;
        for (parent, child) in adjacency_list {
            let parent_id = Uuid::from_str(parent.as_str())
                .map_err(|_| Error::XValueNotOfType("parent not of type uuid"))?;
            let child: Vec<Value> = child.try_into().map_err(|_| Error::XValueNotOfType("child not of type Vec<Value>"))?;
            let child_id : IndexSet<Uuid> = child.into_iter()
                .map(|f| -> Result<Uuid, Error> {
                    return f.try_into()
                        .map_err(|_| Error::XValueNotOfType("cannot convert child to uuid"));
                })
                .take_while(Result::is_ok)
                .map(Result::unwrap)
                .collect();
            tree.adjacency.insert(parent_id, child_id);
        }
        let vertex_vec: Vec<Value> = value
            .remove("vertex")
            .ok_or(crate::Error::XPropertyNotFound("vertices".into()))?
            .try_into()
            .map_err(|_| Error::XValueNotOfType("value"))?;
        for i in vertex_vec {
            let vertex_object: Object =
                i.try_into().map_err(|_| Error::XValueNotOfType("Object"))?;
            let file_node: FileNode = vertex_object
                .try_into()
                .map_err(|_| Error::XValueNotOfType("FileNode"))?;
            tree.vertices.insert(file_node.id, file_node);
        }
        let root: Value = value
            .remove("root")
            .ok_or(crate::Error::XPropertyNotFound("root".into()))?;
        tree.root = match root {
            Value::Strand(x) => {
                Some(uuid::Uuid::from_str(&x).map_err(|_| Error::XValueNotOfType("uuid"))?)
            }
            _ => None,
        };
        Ok(tree)
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
    T: PartialEq + Eq + Clone + Debug,
{
    tree: &'a Tree<ID, T>,
    //stack: HashSet<ID>,
    stack: VecDeque<ID>,
}

impl<'a, ID, T> Iterator for TreeIter<'a, ID, T>
where
    ID: Hash + PartialEq + Eq + Clone + Default + Debug,
    T: PartialEq + Eq + Clone + Debug,
{
    type Item = (&'a ID, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.stack.pop_front() {
            return self.tree.vertices.get_key_value(&x);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn create_tree() -> Tree<i32, String> {
        let mut tree = Tree::new();
        tree.push_vertex(0, "root".to_string());
        tree.push_vertex(1, "first_child".to_string());
        tree.push_edge(0, 1);
        for i in 2..10 {
            tree.push_vertex(i, format!("child {}", i));
            tree.push_edge(i - 1, i);
        }
        return tree;
    }
    #[test]
    fn create_tree_test() {
        assert_eq!(create_tree().vertices, create_tree().vertices);
        assert_eq!(create_tree().adjacency, create_tree().adjacency);
    }
    #[test]
    fn len_test() {
        let tree = create_tree();
        assert_eq!(tree.len_from_start(&0), tree.len());
        assert_eq!(tree.len_from_start(&1), tree.len() - 1);
        assert_eq!(tree.len_from_start(&99), 1);
    }
    #[test]
    fn into_iter_test() {
        let mut tree = create_tree();
        tree.push_children(1, 11, "extra child".to_string());
        //for i in 11..100{
        //tree.push_children(1, i , format!("child : {}", i));
        //}
        for i in tree.into_iter(0) {
            println!("{:?}", i);
        }
    }
}
