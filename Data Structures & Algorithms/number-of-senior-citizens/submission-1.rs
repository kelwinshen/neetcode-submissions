impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        let mut count_senior = 0;

        for i in 0..details.len(){
          let age: i32 =  details[i][details[i].len()-4..details[i].len()-2].to_string().parse::<i32>().unwrap();

          if age > 60 {
            count_senior+=1;
          }
        }

        count_senior
    }
}
