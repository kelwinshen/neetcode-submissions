impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut freq_alphabet = [0; 26];

        for ch in text.bytes(){
            freq_alphabet[(ch-b'a') as usize] +=1;
        }

        let mut count = 0;

        loop {
            if freq_alphabet[(b'b'-b'a') as usize] > 0 {
                freq_alphabet[(b'b'-b'a') as usize] -=1;
            } else {
                break;
            }


             if freq_alphabet[(b'a'-b'a') as usize] > 0 {
                freq_alphabet[(b'a'-b'a') as usize] -=1;
            } else {
                break;
            }


             if freq_alphabet[(b'l'-b'a') as usize] > 1 {
                freq_alphabet[(b'l'-b'a') as usize] -=2;
            } else {
                break;
            }

             if freq_alphabet[(b'o'-b'a') as usize] > 1 {
                freq_alphabet[(b'o'-b'a') as usize] -=2;
            } else {
                break;
            }

             if freq_alphabet[(b'n'-b'a') as usize] > 0 {
                freq_alphabet[(b'n'-b'a') as usize] -=1;
            } else {
                break;
            }

           count+=1;
            
        }

        count
    }
}
