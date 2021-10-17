use std::collections::HashMap;
use nalgebra as math;
use crate::types::*;

type Coord = math::Vector3<i64>;
type NodeId = u64;

pub struct Node<T> {
    pub value : T,
}

pub struct Tree<T> {
    pub nodes : Vec<Node<T>>,
}

// pub trait ChildNodeTrait {
//     type ValueType;
// }

// pub struct Tile<T> {
//     pub value : T,
//     pub active : bool,
// }

// pub struct Node<Child : ChildNodeTrait> {
//     pub child : Child,
//     pub tile : Tile<Child::ValueType>,
// }

// pub struct RootNode<Child : ChildNodeTrait> {
//     pub map : HashMap<Coord, Node<Child>>,
//     pub background_value : Child::ValueType,
// }

// impl<Child : ChildNodeTrait> RootNode<Child> {
//     pub fn get_background_value(&self) -> &Child::ValueType {
//         &self.background_value
//     }
//     pub fn set_background_value(&mut self, new_background_value : Child::ValueType) {
//         self.background_value = new_background_value;
//     }
//     pub fn get_value(&self, x : i64, y : i64, z : i64) -> &Child::ValueType {
//         &self.background_value
//     }
//     pub fn set_value(&mut self, x : i64, y : i64, z : i64, new_value : Child::ValueType) {
//         self.background_value = new_value;
//     }
// }

// pub struct InternalNode {
// }

// pub struct LeafNode<T> {
//     pub 
// }

// pub struct Tree<T> {
//     pub root : RootNode,
// }

// impl<T> Tree<T> {
//     pub fn get_background_value(&self) -> &T {
//         self.root.get_background_value()
//     }
//     pub fn set_background_value(&mut self, new_background_value : T) {
//         self.root.set_background_value(new_background_value);
//     }
//     pub fn get_value(&self, x : i64, y : i64, z : i64) -> &T {
//         &self.root.get_value(x, y, z)
//     }
//     pub fn set_value(&mut self, x : i64, y : i64, z : i64, new_value : T) {
//         self.root.set_value(x, y, z, new_value);
//     }
// }