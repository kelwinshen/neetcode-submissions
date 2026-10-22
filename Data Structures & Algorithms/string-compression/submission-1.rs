impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        chars.push('#');
        let mut latest_char = chars[0];
        let mut count = 1;
        let mut write: usize = 0;
        for read in 1..chars.len(){
            if chars[read] == latest_char {
                count+=1;
            } else {
                 chars[write] = chars[read-1];
                write+=1;
                if count > 1 {
                    let count_to_char_vec: Vec<char> = count.to_string().chars().collect();
                    for k in 0..count_to_char_vec.len(){
                        chars[write] = count_to_char_vec[k];
                        write+=1;
                    }
                }
               
                count = 1;
               
            }
            latest_char = chars[read];
        }

        write as i32
    }
}
