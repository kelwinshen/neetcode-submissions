struct MyHashMap {
    data: Vec<(i32, i32)>
}

impl MyHashMap {
    pub fn new() -> Self {
        MyHashMap{
            data: Vec::new()
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let mut data = &mut self.data;
        for i in 0..data.len(){
            if data[i].0 == key {
                data[i].1 = value;
                return;
            }
        }
        data.push((key, value));
    }

    pub fn get(&self, key: i32) -> i32 {
         let data = &self.data;
        for i in 0..data.len(){
            if data[i].0 == key {
                return data[i].1;
            }
        }

        -1
    }

    pub fn remove(&mut self, key: i32) {
        let mut data = &mut self.data;
        let mut length = data.len();
        let mut write = 0;
        for i in 0..data.len(){
            if data[i].0 != key {
               data[write] = data[i];
                 write+=1;
            }
        }

        for _ in write..length{
            data.pop();
        }

      
    }
}
