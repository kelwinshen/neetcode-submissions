

impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut encoded: String = String::new();
 
        for i in 0..strs.len(){
           encoded.push_str(&strs[i]);
           encoded.push('🥚');
        }


        println!("encoded: {}", encoded);

        encoded
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut encoded_vec: Vec<String> = Vec::new();
        let mut current_string: String = String::new();
        for ch in s.chars(){
            if ch == '🥚' {
                 encoded_vec.push(current_string.clone());
                current_string = String::new();
            } else {
                current_string.push(ch);
            }
        }

        encoded_vec
    }
}
