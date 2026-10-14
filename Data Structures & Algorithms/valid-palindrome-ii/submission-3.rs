impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let b = s.as_bytes();
        let (mut l, mut r) = (0, b.len() - 1);
        while l < r {
            if b[l] != b[r] {
                // one chance: skip left OR skip right
                return Self::is_pal(b, l + 1, r) || Self::is_pal(b, l, r - 1);
            }
            l += 1;
            r -= 1;
        }
        true
    }

    fn is_pal(b: &[u8], mut l: usize, mut r: usize) -> bool {
        while l < r {
            if b[l] != b[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}