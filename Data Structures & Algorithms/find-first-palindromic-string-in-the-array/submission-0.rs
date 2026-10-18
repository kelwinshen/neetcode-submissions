impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for i in 0..words.len(){
           if Self::is_palindrome(&words[i]){
            return words[i].clone();
           }
        }

        String::new()
    }

    pub fn is_palindrome(input: &String)->bool{
        let vec_char: Vec<char> = input.chars().collect();
        let mut left = 0;
        let mut right = vec_char.len() - 1;

        while left < right {
            if vec_char[left] != vec_char[right] {
                return false;
            }
            left+=1;
            right-=1;
        }

        true
    }
}
