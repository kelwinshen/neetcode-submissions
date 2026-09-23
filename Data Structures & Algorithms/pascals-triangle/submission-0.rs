use std::collections::VecDeque;
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
      if num_rows == 1 {
        return vec![vec![1]];
      } else if num_rows == 2 {
           return vec![vec![1], vec![1,1]];
      }


      let mut answers = vec![vec![1], vec![1,1]];

      let mut deepth = 2;

      while deepth < num_rows as usize {
        let mut answer = vec![1];
       
       for i in 1..answers[deepth-1].len(){
        answer.push(answers[deepth-1][i]+answers[deepth-1][i-1]);
       }
       

        answer.push(1);

        answers.push(answer);

           deepth+=1;
      }


    answers


    }
}
