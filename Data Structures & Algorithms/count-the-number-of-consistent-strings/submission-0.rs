impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut allow_array = [false; 256];

        for ch in allowed.bytes(){
            allow_array[(ch-b'a') as usize] = true;
        }

        let mut count = 0;

        for word in words.iter(){
           let mut consistent = true;
            'scope: for ch in word.bytes(){
                if allow_array[(ch-b'a') as usize] == false {
                    consistent = false;
                    break 'scope;
                }
            }

            if consistent {
                count+=1;
            }

  

        }

          count
    }
}