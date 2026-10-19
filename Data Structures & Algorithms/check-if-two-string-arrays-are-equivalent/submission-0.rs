impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
       let word1_concat = word1.concat();
       let word2_concat = word2.concat();

       word1_concat == word2_concat

    }
}
