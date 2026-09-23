impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let chars: Vec<char> = s.chars().collect();
        let mut current: Vec<String> = Vec::new();
        let mut result: Vec<Vec<String>> = Vec::new();

        Self::backtrack(0, &mut current, &chars, &mut result);
        result
    }

    fn is_palindrome(chars: &Vec<char>, mut left: usize, mut right: usize) -> bool {
        while left < right {
            if chars[left] != chars[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }

    fn backtrack(start: usize, current: &mut Vec<String>, chars: &Vec<char>, result: &mut Vec<Vec<String>>) {
        // Base case: we've partitioned the ENTIRE string
        if start == chars.len() {
            result.push(current.clone());
            return;
        }

        // Try every possible substring starting at `start`
        for end in start..chars.len() {
            if Self::is_palindrome(chars, start, end) {
                let substring: String = chars[start..=end].iter().collect();
                current.push(substring);
                Self::backtrack(end + 1, current, chars, result);
                current.pop();
            }
        }
    }
}