impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut words: Vec<String> = s.split_whitespace().map(|a|a.to_string()).collect();

        for i in 0..words.len(){
            let mut left = 0;
            let mut right = words[i].len() - 1;
            let mut vec_word: Vec<char> =  words[i].chars().collect();
            while left < right {
                vec_word.swap(left,right);
                left+=1;
                right-=1;
            }

            words[i] = vec_word.into_iter().collect();
        }


     let answer = words.join(" ");
    answer

    }

   
}
