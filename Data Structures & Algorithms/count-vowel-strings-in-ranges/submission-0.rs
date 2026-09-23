impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut answer: Vec<i32> = Vec::new();

       // queries -> range of words want to be check
 
       for query in queries { //[[0,2],[1,4],[1,1]]
       let mut count = 0;
    
           for i in (query[0] as usize)..=(query[1] as usize){ //[0,2]
                let word = words[i].clone();
                
                if (&word[0..1] == "a" || &word[0..1] == "i" || &word[0..1] == "u" || &word[0..1] == "e" || &word[0..1] == "o") &&  (&word[word.len()-1..word.len()] == "a" || &word[word.len()-1..word.len()] == "i" || &word[word.len()-1..word.len()] == "u" || &word[word.len()-1..word.len()] == "e" || &word[word.len()-1..word.len()] == "o"){
                    count+=1;
                   
                }
           }

           answer.push(count);
       }

      
        answer
    }
}