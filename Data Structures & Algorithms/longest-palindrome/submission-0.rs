impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        //palindrome means read from left to right equal right to left
        
        let mut freq_array = [0; 256];

        for ch in s.bytes(){
            freq_array[ch as usize] +=1;
        }

        let mut length: i32 = 0;

        for i in 0..freq_array.len(){
            while freq_array[i] > 1 {
                freq_array[i] -= 2;
                length+=2;
            }
        }

        if length < s.len() as i32 {
            length +=1;
            return length;
        }

        length


    }
}