impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result: String = String::new();

      let word1_chars: Vec<char> = word1.chars().collect();
      let word2_chars: Vec<char> = word2.chars().collect();

      let mut i = 0;

      while i < word1_chars.len() && i < word2_chars.len(){
        result.push(word1_chars[i]);
        result.push(word2_chars[i]);
        i+=1;
      }

      if i < word1_chars.len() {
        result.push_str(&word1[i..word1_chars.len()]);
      }

        if i < word2_chars.len() {
        result.push_str(&word2[i..word2_chars.len()]);
      }

      result
    }
}
