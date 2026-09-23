

struct MyHashSet {
    data: Vec<i32>
}

impl MyHashSet {
    pub fn new() -> Self {
        MyHashSet{
            data: Vec::new()
        }
    }

    pub fn add(&mut self, key: i32) {
        let mut hashset = &mut self.data;
        if !hashset.contains(&key){
            hashset.push(key);
        }
    }

    pub fn remove(&mut self, key: i32) {
        let mut hashset = &mut self.data;
        let length = hashset.len();
      
        let mut write = 0;
         for i in 0..hashset.len(){
            if hashset[i] != key {
               hashset[write] = hashset[i];
               write+=1;
            }
         }

         for _ in write..length {
            hashset.pop();
         }
        
    }

    pub fn contains(&self, key: i32) -> bool {
         let hashset = &self.data;
        if hashset.contains(&key){
            return true;
        } 

        false
    }
}
