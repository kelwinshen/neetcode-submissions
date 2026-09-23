impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {

        if height.is_empty(){
            return 0;
        }

        let mut left_pointer: usize = 0;
        let mut right_pointer: usize = height.len() - 1;

        let mut max_left_seen: i32 = height[left_pointer];
           let mut max_right_seen: i32 = height[right_pointer];

           let mut total_trapped_water = 0;

           while left_pointer < right_pointer {
                if max_left_seen < max_right_seen {
                    left_pointer+=1;
                    max_left_seen = max_left_seen.max(height[left_pointer]);
                 total_trapped_water+= max_left_seen-height[left_pointer]
                } else{
                    right_pointer-=1;
                      max_right_seen = max_right_seen.max(height[right_pointer]);
                 total_trapped_water+= max_right_seen-height[right_pointer]

                }

           }
        
        total_trapped_water
    }
}
