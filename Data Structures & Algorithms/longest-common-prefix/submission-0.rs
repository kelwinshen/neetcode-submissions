
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
      let mut answer: String = String::new();

    let mut min_length: i32 = i32::MAX;


    for i in 0..strs.len() {
        let s = strs[i].clone();
        if (s.len() as i32) < min_length {
            min_length = s.len() as i32
        }
    }



    'process: for i in 0..min_length as usize {
        let current = &strs[0][i..=i];
       for j in 0..strs.len(){
            if &strs[j][i..=i] !=  current {
                break 'process;       
            }
       }
       answer.push_str(current);
    }
   
      answer


    }
}
