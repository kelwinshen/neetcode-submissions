impl Solution {
  pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    // 1. ALWAYS search the shorter array to prevent Out Of Bounds errors.
    // We swap them if nums1 is longer than nums2.
    let (a, b) = if nums1.len() <= nums2.len() {
        (nums1, nums2)
    } else {
        (nums2, nums1)
    };

    let m = a.len();
    let n = b.len();
    
    let mut left = 0;
    let mut right = m;
    
    // The exact number of elements that should be on the LEFT side of our median.
    // The + 1 ensures that if the total length is ODD, the left side gets the extra number.
    let half_len = (m + n + 1) / 2;

    while left <= right {
        // 2. Make the cut in array 'A' (The Shorter Array)
        let cut_a = left + (right - left) / 2;
        // 3. The cut in array 'B' is mathematically forced!
        let cut_b = half_len - cut_a;

        // 4. Find the 4 numbers immediately touching the cuts.
        // If a cut is at the very edge of an array, we use MIN/MAX as invisible boundaries.
        let max_left_a = if cut_a == 0 { i32::MIN } else { a[cut_a - 1] };
        let min_right_a = if cut_a == m { i32::MAX } else { a[cut_a] };

        let max_left_b = if cut_b == 0 { i32::MIN } else { b[cut_b - 1] };
        let min_right_b = if cut_b == n { i32::MAX } else { b[cut_b] };

        // 5. Check if the cut is perfect!
        if max_left_a <= min_right_b && max_left_b <= min_right_a {
            
            // WE FOUND IT! Now just do the math based on if the total length is Even or Odd.
            if (m + n) % 2 == 0 {
                // EVEN: Average of the largest on the left and the smallest on the right
                let left_max = max_left_a.max(max_left_b) as f64;
                let right_min = min_right_a.min(min_right_b) as f64;
                return (left_max + right_min) / 2.0;
            } else {
                // ODD: The median is just the largest number on the left!
                return max_left_a.max(max_left_b) as f64;
            }
            
        } else if max_left_a > min_right_b {
            // Array A's left side is too big. We need to move the cut to the left.
            right = cut_a - 1;
        } else {
            // Array A's left side is too small. We need to move the cut to the right.
            left = cut_a + 1;
        }
    }
    
    0.0 // Rust requires a return here, but valid inputs will never reach this.
}
}
