impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut left_pointer = 0;
        let mut right_pointer = heights.len()-1;

        //max area is max of (i-j) multiple by max of min(height i, height j)
        let mut max_area: i32 = 0;
        while left_pointer < right_pointer {
            let area = (right_pointer-left_pointer) as i32 * heights[left_pointer].min(heights[right_pointer]);
            if area > max_area {
                max_area = area as i32;
            }

            if heights[left_pointer] > heights[right_pointer] {
                right_pointer-=1;
            } else {
                left_pointer+=1;
            }
        }

        max_area
    }
}
