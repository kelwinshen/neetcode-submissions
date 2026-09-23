impl Solution {
    pub fn max_score(s: String) -> i32 {
       let mut left_vec: Vec<i32> = Vec::new();
        let mut count = 0;
        let mut i = 0;
       for ch in s.chars(){
            if i == s.len()-1{
                break;
            }
            if ch == '0' {
                count+=1;
            }
            left_vec.push(count);
            i+=1;
       }

       println!("{:?}", left_vec);

        let mut right_vec: Vec<i32> = Vec::new();
        let mut count = 0;
         let mut i = 0;
       for ch in s.chars().rev(){
             if i == s.len()-1 {
                break;
            }
            if ch == '1' {
                count+=1;
            }
            right_vec.push(count);
            i+=1;
          
       }

         println!("{:?}", right_vec);

       let mut max_sum = 0;

    

       for i in 0..left_vec.len(){
        max_sum = max_sum.max(left_vec[i]+right_vec[right_vec.len()-1-i]);
       }

     

       max_sum



      
    }
}