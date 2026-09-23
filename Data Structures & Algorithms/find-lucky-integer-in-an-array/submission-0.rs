use std::collections::HashMap;
impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut hash_freq: HashMap<i32, i32> = HashMap::new();

        for i in 0..arr.len(){
            *hash_freq.entry(arr[i]).or_insert(0) +=1;
        }

        let mut largest_lucky_number = -1;

        for (val, freq) in hash_freq.iter(){
            if val == freq {
                largest_lucky_number = largest_lucky_number.max(*val);
            }
        }

       largest_lucky_number
    }
}
