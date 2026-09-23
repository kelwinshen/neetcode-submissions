use std::collections::{HashMap, HashSet};

impl Solution {
   pub fn largest_good_integer(num: String) -> String {
        let bytes = num.as_bytes();
        let mut max_char: Option<u8> = None;

        // Check every window of 3 consecutive characters
        for window in bytes.windows(3) {
            if window[0] == window[1] && window[1] == window[2] {
                // Keep track of the largest character digit found
                max_char = max_char.max(Some(window[0]));
            }
        }

        // Repeat the largest char 3 times, or return empty string
        match max_char {
            Some(ch) => (ch as char).to_string().repeat(3),
            None => String::new(),
        }
    }
}
