struct NumArray {
    nums: Vec<i32>
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        NumArray{
            nums
        }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let mut sum = 0;
        let nums = &self.nums;
        let mut current = 0;
        for i in (left as usize)..=(right as usize) {
            sum += nums[i];
        }

        sum
    }
}
