impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut result: Vec<char> = Vec::new();

        let mut vec_char_string: Vec<char> = s.chars().collect();
        
        let mut current_space: usize = 0;

        for i in 0..vec_char_string.len(){
            if current_space < spaces.len() && i == spaces[current_space] as usize {
                result.push(' ');
                current_space += 1;
            }

              result.push(vec_char_string[i]);


        }

       let answer: String = result.into_iter().collect();

answer
        

    }
}
