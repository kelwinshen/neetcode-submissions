impl Solution {
    pub fn length_of_last_word(mut s: String) -> i32 {

      let mut vec_string: Vec<_> = s.split_whitespace().collect();



      for i in 0..vec_string.len() {
        let string = vec_string[vec_string.len()-i-1];

        if string.len() > 0 {
            return string.len() as i32;
        }
      }

      0
    }
}
