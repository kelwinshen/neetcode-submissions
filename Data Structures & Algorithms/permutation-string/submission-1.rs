use std::collections::HashMap;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {

        //check does any substring in combination of s1 in s2 

        let mut freq_s1: [u8; 26] = [0; 26];

        
        let bytes_s1 = s1.as_bytes();
        let bytes_s2 = s2.as_bytes();

        for i in 0..bytes_s1.len(){
            freq_s1[(bytes_s1[i] - b'a') as usize] +=1;
        }
        
        let mut start: usize  = 0;

        while start + bytes_s1.len() <= bytes_s2.len() {
              let mut freq_s2: [u8; 26] = [0; 26];
            
               for i in start..start + bytes_s1.len(){
            freq_s2[(bytes_s2[i] - b'a') as usize] +=1;
        }


           if freq_s2 == freq_s1 {
            return true;
           }

           start+=1;


        }


    

        
      
false
    
    }
}
