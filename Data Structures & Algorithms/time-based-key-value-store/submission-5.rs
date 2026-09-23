use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TimeMap{
    pub data: HashMap<String, Vec<(String, i32)>>
}

impl TimeMap{
    pub fn new()->Self{
        TimeMap {
            data: HashMap::new(),
        }
    }
    
    pub fn set(&mut self, key: String, value: String, timestamp: i32){
        self.data.entry(key).or_insert(Vec::new()).push((value, timestamp));
    }
    
     pub fn get(&self, key: String, timestamp: i32)->String{
// String get(String key, int timestamp) Returns a value such that set was called previously, with timestamp_prev <= timestamp. If there are multiple such values, it returns the value associated with the largest timestamp_prev. If there are no values, it returns "".       
        match self.data.get(&key){
            Some(vector_string)=>{
                
                let mut candidates: Vec<(String, i32)> = Vec::new();
                
                let mut left = 0;
                let mut right = vector_string.len();
                
                while left < right {
                    let middle = left + (right-left)/2;
                    if vector_string[middle].1 == timestamp {
                        return vector_string[middle].0.clone();
                    } else if vector_string[middle].1 < timestamp {
                        candidates.push(vector_string[middle].clone());
                        left = middle+1;
                    } else {
                        right = middle;
                    }
                }
                
                 if !candidates.is_empty(){
                     return candidates.last().unwrap().0.clone();
                 } else {
                     return String::new();
                 }
                
                
                
            },
            None => {}
        }
        
        String::new()
    }
    
    
    
}