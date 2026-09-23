use std::collections::HashSet;

struct MyHashSet {
    data: HashSet<i32>
}

impl MyHashSet {
    pub fn new() -> Self {
        MyHashSet{
            data: HashSet::new()
        }
    }

    pub fn add(&mut self, key: i32) {
        let mut hashset = &mut self.data;
        hashset.insert(key);
    }

    pub fn remove(&mut self, key: i32) {

        let mut hashset = &mut self.data;
        hashset.remove(&key);
    }

    pub fn contains(&self, key: i32) -> bool {
         let hashset = &self.data;
        if hashset.get(&key).is_some(){
            return true;
        } 

        false
    }
}
