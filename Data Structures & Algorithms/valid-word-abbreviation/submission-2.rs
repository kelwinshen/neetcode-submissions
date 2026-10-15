impl Solution {
    pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
        let w = word.as_bytes();
        let a = abbr.as_bytes();
        let mut i = 0usize; // position in word
        let mut j = 0usize; // position in abbr

        while j < a.len() {
            if a[j].is_ascii_digit() {
                if a[j] == b'0' {
                    return false; // leading zero (or "0" alone) is not allowed
                }
                let mut num = 0usize;
                while j < a.len() && a[j].is_ascii_digit() {
                    num = num * 10 + (a[j] - b'0') as usize;
                    j += 1;
                }
                i += num; // skip that many letters in word
            } else {
                if i >= w.len() || w[i] != a[j] {
                    return false;
                }
                i += 1;
                j += 1;
            }
        }
        i == w.len() // both must finish together
    }
}