impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        // O(1) extra memory.
        let length = s.len();
        for i in 0..s.len()/2{
              s.swap(i, length-1-i);
        }
      
    }
}
