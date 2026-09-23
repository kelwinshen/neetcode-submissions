impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut vec_char_order:  Vec<char>  = order.chars().collect();

        let mut vec_char_s: Vec<char> =  s.chars().collect();


        let mut write = 0;

        for i in 0..vec_char_order.len(){
            for j in 0..vec_char_s.len(){
                if vec_char_s[j] == vec_char_order[i] {
                    let temp = vec_char_s[write];
                    vec_char_s[write] = vec_char_order[i];
                    vec_char_s[j] = temp;
                    write+=1;
                }
            }
        }

        vec_char_s.into_iter().collect()
    }
}
