impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut score = 0;
        let mut last_bytes = 0;
        let mut i = 0;
        for ch in s.bytes(){
            
           if i != 0 {
           
                score+= ((ch as i32)-(last_bytes as i32)).abs();
              
           }

         
           last_bytes = ch;
           i+=1;
          
        }

        score
    }
}
