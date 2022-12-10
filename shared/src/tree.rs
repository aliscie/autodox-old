use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;
use uuid::Uuid;

#[cfg(feature = "backend")]
use speedy::{Readable, Writable};

#[cfg(feature = "tauri")]
use surrealdb::sql::{Array, Object, Value};

use crate::id::Id;

use crate::schema::FileNode;
use crate::traits::{Entity, GetId};
use crate::Error;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Tree<ID, T>
where
    ID: Hash + PartialEq + Eq + Clone + Default + Debug,
    T: PartialEq + Eq + Clone + Debug,
{
    pub vertices: HashMap<ID, T>,
    pub adjacency: HashMap<ID, HashSet<ID>>,
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
        adjacency.remove(&child_id);
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

    pub fn remove(&mut self, id: &ID) -> ID {
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
        let mut parent_id = ID::default();
        for (_id, children) in self.adjacency.iter_mut() {
            if children.remove(id) {
                parent_id = _id.clone();
            }
        }
        parent_id
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

    fn find_path_inner(&self, start: &ID, id: &ID, path: &mut Vec<ID>) -> bool {
        path.push(start.clone());
        if start == id {
            // base case
            return true;
        }
        if let Some(children) = self.adjacency.get(start) {
            for i in children {
                if self.find_path_inner(i, id, path) {
                    // found the node
                    return true;
                }
            }
        }
        path.pop();
        false
    }

    pub fn find_path(&self, start: &ID, id: &ID) -> Vec<ID> {
        let mut path = Vec::new();
        self.find_path_inner(start, id, &mut path);
        path
    }
}

#[cfg(feature = "tauri")]
impl<ID, T> From<Tree<ID, T>> for Object
where
    ID: Hash + PartialEq + Eq + Clone + Default + Debug + Into<Value>,
    T: PartialEq + Eq + Clone + Debug + Into<Object> + Entity,
{
    fn from(val: Tree<ID, T>) -> Object {
        let mut map = BTreeMap::new();
        let mut vertices_vec: Vec<Value> = Vec::new();
        for i in val.vertices.keys() {
            vertices_vec.push(Value::Thing((T::table_name(), format!("{:?}", i)).into()));
        }
        map.insert("vertices".to_owned(), Value::Array(vertices_vec.into()));
        map.insert(
            "root".to_owned(),
            val.root.map(|f| format!("{:?}", f)).into(),
        );
        return map.into();
    }
}

impl GetId for FileNode {
    type Id = Id;
    fn get_id(&self) -> Self::Id {
        self.id
    }
}

// cannot make it generic over Tree<ID, T> since Id is conversion
// is problematic with surrealdb
#[cfg(feature = "tauri")]
impl<T> TryFrom<Object> for Tree<Id, T>
where
    T: PartialEq + Eq + Clone + Debug + TryFrom<Object, Error = Error> + Entity + Serialize + GetId<Id = Id>,
{
    type Error = crate::Error;
    /// i am asuming we have selected the vertices field with all the data in the nodes
    fn try_from(value: Object) -> Result<Self, Self::Error> {
        let mut value = value;
        let mut tree: Tree<Id, T> = Tree::new();
        let vertex_vec: Vec<Value> = value
            .remove("vertices")
            .ok_or(crate::Error::XPropertyNotFound("vertices".into()))?
            .try_into()
            .map_err(|_| Error::XValueNotOfType("vertices not of type Value"))?;
        for i in vertex_vec {
            let mut vertex_object: Object =
                i.try_into().map_err(|_| Error::XValueNotOfType("vertices not of type Value::Object"))?;
            let adjacency_array: Vec<Value> = vertex_object
                .remove("children")
                .ok_or(Error::XPropertyNotFound("children not found".into()))?
                .try_into()
                .map_err(|_| Error::XValueNotOfType("file_node.children not of type Array"))?;
            let adjacency: HashSet<Id> = adjacency_array
                .into_iter()
                .filter_map(|e| -> Option<Id> { e.record()?.id.to_raw().as_str().try_into().ok() })
                .collect();
            let file_node: T = vertex_object
                .try_into()?;
            tree.adjacency.insert(file_node.get_id(), adjacency);
            tree.vertices.insert(file_node.get_id(), file_node);
        }
        let root: Value = value
            .remove("root")
            .ok_or(crate::Error::XPropertyNotFound("root".into()))?;
        tree.root = match root {
            Value::Strand(x) => Some(
                uuid::Uuid::from_str(&x)
                    .map_err(|_| Error::XValueNotOfType("uuid"))?
                    .into(),
            ),
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

#[cfg( feature = "backend")]
impl<
    ID: candid::types::CandidType,
    T: candid::types::CandidType,
> candid::types::CandidType for Tree<ID, T>
where
    ID: Hash + PartialEq + Eq + Clone + Default + Debug,
    T: PartialEq + Eq + Clone + Debug,
{
    fn _ty() -> candid::types::Type {
        candid::types::Type::Record(
            <[_]>::into_vec(
                Box::new([
                    candid::types::Field {
                        id: candid::types::Label::Named("root".to_string()),
                        ty: <Option<ID> as candid::types::CandidType>::ty(),
                    },
                    candid::types::Field {
                        id: candid::types::Label::Named("vertices".to_string()),
                        ty: <HashMap<ID, T> as candid::types::CandidType>::ty(),
                    },
                    candid::types::Field {
                        id: candid::types::Label::Named("adjacency".to_string()),
                        ty: <HashMap<
                            ID,
                            HashSet<ID>,
                        > as candid::types::CandidType>::ty(),
                    },
                ]),
            ),
        )
    }
    fn id() -> candid::types::TypeId {
        candid::types::TypeId::of::<Tree<ID, T>>()
    }
    fn idl_serialize<__S>(
        &self,
        __serializer: __S,
    ) -> std::result::Result<(), __S::Error>
    where
        __S: candid::types::Serializer,
    {
        let mut ser = __serializer.serialize_struct()?;
        candid::types::Compound::serialize_element(&mut ser, &self.root)?;
        candid::types::Compound::serialize_element(&mut ser, &self.vertices)?;
        candid::types::Compound::serialize_element(&mut ser, &self.adjacency)?;
        Ok(())
    }
}

#[cfg( feature = "backend")]
impl<'a_, ID, T, C_: speedy::Context> speedy::Readable<'a_, C_> for Tree<ID, T>
where
    ID: speedy::Readable<'a_, C_>,
    T: speedy::Readable<'a_, C_>,
    ID: speedy::Readable<'a_, C_>,
    HashSet<ID>: speedy::Readable<'a_, C_>,
    Option<ID>: speedy::Readable<'a_, C_>,
    ID: Hash + PartialEq + Eq + Clone + Default + Debug,
    T: PartialEq + Eq + Clone + Debug,
{
    #[inline]
    fn read_from<R_: speedy::Reader<'a_, C_>>(
        _reader_: &mut R_,
    ) -> std::result::Result<Self, C_::Error> {
        let vertices: HashMap<ID, T> = {
            let _length_ = speedy::private::read_length_u32(_reader_)?;
            _reader_.read_key_value_collection(_length_)
        }?;
        let adjacency: HashMap<ID, HashSet<ID>> = {
            let _length_ = speedy::private::read_length_u32(_reader_)?;
            _reader_.read_key_value_collection(_length_)
        }?;
        let root: Option<ID> = {
            _reader_
                .read_u8()
                .and_then(|_flag_| {
                    if _flag_ != 0 { Ok(Some(_reader_.read_value()?)) } else { Ok(None) }
                })
        }?;
        Ok(Tree { vertices, adjacency, root })
    }
    #[inline]
    fn minimum_bytes_needed() -> usize {
        {
            let mut out = 0;
            out += 4usize;
            out += 4usize;
            out += 1;
            out
        }
    }
}

#[cfg(feature = "backend")]
impl<ID, T, C_: speedy::Context> speedy::Writable<C_> for Tree<ID, T>
where
    ID: speedy::Writable<C_>,
    T: speedy::Writable<C_>,
    ID: speedy::Writable<C_>,
    HashSet<ID>: speedy::Writable<C_>,
    Option<ID>: speedy::Writable<C_>,
    ID: Hash + PartialEq + Eq + Clone + Default + Debug,
    T: PartialEq + Eq + Clone + Debug,
{
    #[inline]
    fn write_to<T_: ?Sized + speedy::Writer<C_>>(
        &self,
        _writer_: &mut T_,
    ) -> std::result::Result<(), C_::Error> {
        let vertices = &self.vertices;
        let adjacency = &self.adjacency;
        let root = &self.root;
        {
            speedy::private::write_length_u32(vertices.len(), _writer_)?;
            _writer_.write_collection(vertices.iter())?;
        }
        {
            speedy::private::write_length_u32(adjacency.len(), _writer_)?;
            _writer_.write_collection(adjacency.iter())?;
        }
        {
            if let Some(ref root) = root {
                _writer_.write_u8(1)?;
                _writer_.write_value(root)?;
            } else {
                _writer_.write_u8(0)?;
            }
        }
        Ok(())
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
    #[test]
    fn find_path_test() {
        let tree = create_tree();
        println!("{:?}", tree);
        println!("path is : {:?}", tree.find_path(&0, &9));
    }
}
