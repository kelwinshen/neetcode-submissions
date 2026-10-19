impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let s_vec: Vec<char> = s.chars().collect();
        let t_vec: Vec<char> = t.chars().collect();

        let mut s_answer: Vec<char> = Vec::new();
        for i in 0..s_vec.len(){
            if s_vec[i] == '#' && !s_answer.is_empty(){
                s_answer.pop();
            } else if s_vec[i] != '#' {
                s_answer.push(s_vec[i]);
            }
        }

        let mut t_answer: Vec<char> = Vec::new();
         for i in 0..t_vec.len(){
            if t_vec[i] == '#' && !t_answer.is_empty(){
                t_answer.pop();
            }else if t_vec[i] != '#' {
                t_answer.push(t_vec[i]);
            }
        }

        s_answer == t_answer
    }
}
