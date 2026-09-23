impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {

       //t.len() >= s.len()


       let s_bytes = s.as_bytes();
       let t_bytes = t.as_bytes();
       let mut j: usize = 0;
       let mut i: usize = 0;

       while i < t_bytes.len() && j < s_bytes.len() {
            if s_bytes[j] == t_bytes[i] {
                j+=1;
            }
            i+=1;
       }

       j == s_bytes.len()
    }
}
