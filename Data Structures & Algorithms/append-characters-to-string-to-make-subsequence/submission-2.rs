impl Solution {
   pub fn append_characters(s: String, t: String) -> i32 {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        let mut i = 0;
        let mut j = 0;

        while i < s_bytes.len() && j < t_bytes.len() {
            if s_bytes[i] == t_bytes[j] {
                j += 1;
            }
            i += 1;
        }

        (t_bytes.len() - j) as i32
    }
}
