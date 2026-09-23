impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut freq = [0; 256];

        for ch in chars.bytes() {
            freq[(ch - b'a') as usize] += 1;
        }

        let mut count = 0;

        for word in words {
            let mut freq_of_char = freq;
            let mut is_good = true;

            for ch in word.bytes() {
                let idx = (ch - b'a') as usize;
                if freq_of_char[idx] > 0 {
                    freq_of_char[idx] -= 1;
                } else {
                    is_good = false;
                    break; // Unlabeled break exits the inner loop naturally
                }
            }

            if is_good {
                count += word.len() as i32;
            }
        }

        count
    }
}