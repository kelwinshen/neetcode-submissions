use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut hashmap: HashMap<char, Vec<i32>> = HashMap::new();
        let mut hashset: HashSet<char> = HashSet::new();

        let mut i: i32 = 0;
        for ch in s.chars(){
            hashmap.entry(ch).or_insert(Vec::new()).push(i);
            hashset.insert(ch);
            i +=1;
        }

        let mut answer = -1;

        for ch in hashset.iter() {
            let mut vec_index = hashmap.get(&ch).unwrap();
            if vec_index.len() > 1 {
              answer = answer.max(vec_index[vec_index.len()-1] - vec_index[0]-1);
            }

         
        }

        answer
    }
}