use std::collections::HashMap;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {

        let mut hashmap_index: HashMap<u8, Vec<i32>> = HashMap::new();

        let bytes = s.as_bytes();

        for i in 0..bytes.len() {
            hashmap_index.entry(bytes[i]).or_insert(Vec::new()).push(i as i32);
        }
        //XYYX
        //x-> [0, 3]-> 3-0 + 1 = 2 
        //AAABABB
        //0,1,2,4

       let mut max_len = 0;

    // 2. For each character's list of indices...
    for (_b, indexes) in hashmap_index {
        let mut left = 0;

        for right in 0..indexes.len() {
            // How many characters are strictly between the start and end of our window?
            let total_distance = indexes[right] - indexes[left] + 1;
            let good_characters = right - left + 1;
            
            // The bad characters are the gaps we need to fill!
            let replacements_needed = total_distance - good_characters as i32;

            // If the gaps are bigger than our budget `k`, shrink from the left
            while (indexes[right] - indexes[left] + 1) as i32  - (right - left + 1) as i32  > k  {
                left += 1;
            }

            // The maximum length we can make is the good characters + our k budget.
            // (We use min() because we can't make a string longer than the original `s`)
            let possible_len = std::cmp::min(bytes.len(), (right - left + 1) + k as usize);
            
            if possible_len > max_len {
                max_len = possible_len;
            }
        }
    }

    max_len as i32

       
    }
}
