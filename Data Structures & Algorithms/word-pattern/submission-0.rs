use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let pattern_bytes = pattern.as_bytes();
        let words: Vec<&str> = s.split_whitespace().collect();

        // Lengths must match first
        if pattern_bytes.len() != words.len() {
            return false;
        }

        let mut p_last_seen: HashMap<u8, usize> = HashMap::new();
        let mut w_last_seen: HashMap<&str, usize> = HashMap::new();

        for i in 0..words.len() {
            // insert returns Option<usize> (the previous value at that key)
            let p_pos = p_last_seen.insert(pattern_bytes[i], i + 1);
            let w_pos = w_last_seen.insert(words[i], i + 1);

            // If the last time we saw this char and word don't match, pattern is broken
            if p_pos != w_pos {
                return false;
            }
        }

        true
    }
}
