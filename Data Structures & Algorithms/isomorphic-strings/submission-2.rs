impl Solution {
   pub fn is_isomorphic(s: String, t: String) -> bool {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        let mut s_last_seen: HashMap<u8, usize> = HashMap::new();
        let mut t_last_seen: HashMap<u8, usize> = HashMap::new();

        for i in 0..s_bytes.len() {
            // Check the last position where s[i] and t[i] appeared (+1 to distinguish from 0/None)
            let s_pos = s_last_seen.insert(s_bytes[i], i + 1);
            let t_pos = t_last_seen.insert(t_bytes[i], i + 1);

            // If their last-seen positions don't match, mapping is broken
            if s_pos != t_pos {
                return false;
            }
        }

        true
    }
}
