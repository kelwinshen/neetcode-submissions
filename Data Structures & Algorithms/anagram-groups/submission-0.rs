use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut freq_hash: HashMap<Vec<i32>, Vec<String>> = HashMap::new();

        for string in strs{
            let mut freq_str: Vec<i32> = vec![0; 26];
            for b in string.bytes(){
                freq_str[(b - b'a') as usize] += 1;
            }

            freq_hash.entry(freq_str).or_insert(Vec::new()).push(string);
        }     

        let mut result: Vec<Vec<String>> = Vec::new();

        for (key,sublist) in freq_hash{
            result.push(sublist);
        }

        result


    }
}
