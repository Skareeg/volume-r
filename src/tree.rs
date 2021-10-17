pub struct RootNode {
}

pub struct InternalNode {
}

pub struct LeafNode {
}

pub struct Tree<T> {
    pub root : RootNode,
    pub background_value : T,
}

impl<T> Tree<T> {
    pub fn get_background_value(&self) -> &T {
        &self.background_value
    }
    pub fn set_background_value(&mut self, new_background_value : T) {
        self.background_value = new_background_value;
    }
    pub fn get_value(&self, x : i64, y : i64, z : i64) -> &T {
        &self.background_value
    }
    pub fn set_value(&mut self, x : i64, y : i64, z : i64, new_value : T) {
        self.background_value = new_value;
    }
}