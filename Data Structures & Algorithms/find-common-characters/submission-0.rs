impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut min_freq = [usize::MAX; 26];

        for word in &words {
            let mut freq = [0; 26];
            for ch in word.bytes() {
                freq[(ch - b'a') as usize] += 1;
            }
            for i in 0..26 {
                min_freq[i] = min_freq[i].min(freq[i]);
            }
        }

        let mut answers = Vec::new();
        for i in 0..26 {
            for _ in 0..min_freq[i] {
                answers.push(((i as u8 + b'a') as char).to_string());
            }
        }

        answers
    }
}