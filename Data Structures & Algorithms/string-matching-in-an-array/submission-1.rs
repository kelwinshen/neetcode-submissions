impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        
        let mut answers: Vec<String> = Vec::new();

        for i in 0..words.len(){
            let current = &words[i];
           
         'scope: for j in 0..words.len(){
                   let mut start = 0;
                   while start + current.len() <= words[j].len() && i != j {
                     
                  
                if current == &words[j][start..start+current.len()] {
                   
                    answers.push(current.clone());
                    break 'scope;
                   
                }
                start+=1;
            }
            }
         
            
        }
    

        answers
    }

    


}
