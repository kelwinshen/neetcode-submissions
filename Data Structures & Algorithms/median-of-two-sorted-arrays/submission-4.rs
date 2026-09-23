impl Solution {
  fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (a, b) = if nums1.len() <= nums2.len() {
        (&nums1, &nums2)
    } else {
        (&nums2, &nums1)
    };

    let m = a.len();
    let n = b.len();
    let half_left = (m + n + 1) / 2;

    let mut left = 0;
    let mut right = m;

    loop {
        let x = left + (right - left) / 2;
        let y = half_left - x;

        let left_x = if x == 0 { i32::MIN } else { a[x - 1] };
        let right_x = if x == m { i32::MAX } else { a[x] };
        let left_y = if y == 0 { i32::MIN } else { b[y - 1] };
        let right_y = if y == n { i32::MAX } else { b[y] };

        if left_x <= right_y && left_y <= right_x {
            if (m + n) % 2 == 1 {
                return left_x.max(left_y) as f64;
            } else {
                return (left_x.max(left_y) as f64 + right_x.min(right_y) as f64) / 2.0;
            }
        } else if left_x > right_y {
            right = x - 1;
        } else {
            left = x + 1;
        }
    }
}
}
