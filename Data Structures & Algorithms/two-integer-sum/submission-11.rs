

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        //we have to remember the original index
        //we can use something like tupple that store the nums and the index 
        //so we sort without losing the original index
        //after that we can use two pointer to find the target we sort
        //after we find we can use the tuple that store the index to return then 

        let mut tupple_vec: Vec<(i32, usize)> = Vec::new();
        for (i,num) in nums.into_iter().enumerate() {
            tupple_vec.push((num, i));
        }

        tupple_vec.sort_by(|a,b| a.0.cmp(&b.0));

        let mut left: usize = 0;
        let mut right: usize = tupple_vec.len()-1;

        while left < right {
           if tupple_vec[left].0 + tupple_vec[right].0 == target {
            let mut answer = vec![tupple_vec[left].1 as i32, tupple_vec[right].1 as i32];
            answer.sort();
                return answer;
           } else if tupple_vec[left].0 + tupple_vec[right].0 > target {
               right-=1;
           } else {
             left+=1;
           }
        }

        Vec::new()
    }
}
