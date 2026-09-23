use std::collections::HashMap;

struct TimeMap {
   pub data: HashMap<String, Vec<(i32, String)>>
}

impl TimeMap {
    fn new() -> Self {
        TimeMap{
            data: HashMap::new()
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.data.entry(key).or_insert(Vec::new()).push((timestamp, value));
     }

    fn get(&self, key: String, timestamp: i32) -> String {
        match self.data.get(&key) {
           Some(vec_data)=>{
                let mut start: usize = 0;
                let mut end: usize = vec_data.len();
                let mut candidates: Vec<usize> = Vec::new();
                while start < end {
                    let middle = start + (end-start)/2;
                    if vec_data[middle].0 == timestamp {
                        return vec_data[middle].1.clone();
                    } else if vec_data[middle].0 < timestamp {
                        candidates.push(middle);
                        start = middle + 1;
                    } else {
                        end = middle;
                    }
                }

                if !candidates.is_empty(){
                     return (vec_data[*candidates.last().unwrap()]).1.clone();
                } else {
                     return String::new();
                }

                
           },
           None=>{
            return String::new()
           }
        }
        
    }
}
