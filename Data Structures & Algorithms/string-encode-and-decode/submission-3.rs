

impl Solution {
pub fn encode(strs: Vec<String>)->String{
    let mut encoded: String = String::new();
    
    for i in 0..strs.len(){
        encoded.push_str(&strs[i]);
          encoded.push_str("🥚");
    }
    
    encoded
    
}

pub fn decode(encoded_strs: String)->Vec<String>{
    let mut decoded_vec: Vec<String> = Vec::new();
    
    let mut current_formed_string = String::new();
    
    for ch in encoded_strs.chars(){
        if ch == '🥚' {
            decoded_vec.push(current_formed_string.clone());
            current_formed_string = String::new();
        } else {
            current_formed_string.push(ch);
        }
    }
    
    decoded_vec
    
    
}
}
