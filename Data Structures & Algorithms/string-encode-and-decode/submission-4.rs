

impl Solution {
pub fn encode(strs: Vec<String>)->String{
    let mut encoded: String = String::new();
        for i in 0..strs.len(){
            encoded.push_str(&strs[i]);
            encoded.push('😋');
        }
        
        encoded
    }
    
    pub fn decode(encoded_string: String)->Vec<String>{
        let mut strs: Vec<String> = encoded_string.split('😋').map(|value|value.to_string()).collect();
        strs.pop();
        strs
    }
}
