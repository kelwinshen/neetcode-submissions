impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
       //convert digits to a vector integer
       //map each integer to a set of string
       //using back tracking recursive to find the combinations of each group then return
    if digits.len() == 0 {
        return Vec::new();
    }
        let mut vec_int: Vec<i32> = Vec::new();
       for ch in digits.chars(){
            vec_int.push(ch.to_digit(10).unwrap() as i32);
       }

      let phone_map: [&str; 8] = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut groups: Vec<Vec<String>> = Vec::new();
        for num in vec_int {
            let letters = phone_map[(num - 2) as usize];
            let group: Vec<String> = letters.chars().map(|c| c.to_string()).collect();
            groups.push(group);
        }

       let mut current: Vec<String> = Vec::new();
        let mut results: Vec<String> = Vec::new();

        Self::back_tracking(0, &mut current, &groups, &mut results);
        results


    }

     pub fn back_tracking(
        index: usize,
        current: &mut Vec<String>,
        groups: &Vec<Vec<String>>,
        results: &mut Vec<String>,
    ) {
        if index == groups.len() {
            results.push(current.concat()); // join current pieces into one String
            return;
        }

        for letter in &groups[index] {
            current.push(letter.clone());
            Self::back_tracking(index + 1, current, groups, results);
            current.pop();
        }
    }
}
