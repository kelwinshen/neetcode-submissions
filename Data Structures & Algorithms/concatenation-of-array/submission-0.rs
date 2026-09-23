impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut answer = nums.clone();
        answer.extend(nums);
        answer
    }
}
