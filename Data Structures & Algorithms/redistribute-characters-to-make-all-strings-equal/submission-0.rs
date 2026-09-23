impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut freq_char = [0; 26];

        for i in 0..words.len(){
            for ch in words[i].bytes(){
                freq_char[(ch-b'a') as usize] +=1;
            }
        }

        for i in 0..freq_char.len(){
            if freq_char[i] % words.len() as i32 != 0 {
                return false;
            }
        }

        true
    }   
}