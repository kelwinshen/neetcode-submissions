use std::collections::HashMap;

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut hashmap_freq: HashMap<char, i32> = HashMap::new();

        for ch in s.chars(){
            *hashmap_freq.entry(ch).or_insert(0) += 1;
        }

        let mut max_odd = 0;
        let mut max_even = i32::MAX;

        for (&ch, &freq) in  hashmap_freq.iter(){
            if freq % 2 == 0 {
                if max_even > freq {
                    max_even = freq;
                 }
            } else {
                 if max_odd < freq {
                    max_odd = freq;
                 }
            }
        }

    max_odd - max_even
       

     
    }
}
