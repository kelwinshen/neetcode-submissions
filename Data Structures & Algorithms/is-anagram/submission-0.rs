impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut freq_vec_s: Vec<i32> = vec![0; 26];
        let mut freq_vec_t: Vec<i32> = vec![0; 26];

        for b in s.bytes(){
            freq_vec_s[(b-b'a') as usize] +=1;
        }


         for b in t.bytes(){
            freq_vec_t[(b-b'a') as usize] +=1;
        }

        freq_vec_s == freq_vec_t


    }
}
