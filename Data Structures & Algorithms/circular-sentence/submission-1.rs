impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let bytes = sentence.as_bytes();
        let n = bytes.len();

        // The first and last characters of the sentence must match
        if bytes[0] != bytes[n - 1] {
            return false;
        }

        // Check every space boundary
        for i in 0..n {
            if bytes[i] == b' ' {
                // The char before the space must match the char after the space
                if bytes[i - 1] != bytes[i + 1] {
                    return false;
                }
            }
        }

        true
    }
}