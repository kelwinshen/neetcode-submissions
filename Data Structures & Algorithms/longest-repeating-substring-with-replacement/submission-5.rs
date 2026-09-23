use std::collections::HashMap;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s_bytes = s.as_bytes();
        let mut freq: HashMap<u8, i32> = HashMap::new();
        let mut left = 0;
        let mut max_freq = 0;
        let mut max_len = 0;

        for right in 0..s_bytes.len() {
            *freq.entry(s_bytes[right]).or_insert(0) += 1;
            max_freq = max_freq.max(freq[&s_bytes[right]]);

            let window_len = (right - left + 1) as i32;
            if window_len  > k + max_freq{
                *freq.get_mut(&s_bytes[left]).unwrap() -= 1;
                left += 1;
            }

            max_len = max_len.max((right - left + 1) as i32);
        }

        max_len
    }
}