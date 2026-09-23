impl Solution {
 fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let total = nums1.len() + nums2.len();
    let mut i = 0;
    let mut j = 0;
    let mut prev = 0;
    let mut curr = 0;

    for _ in 0..=total / 2 {
        prev = curr;
        if i < nums1.len() && (j >= nums2.len() || nums1[i] <= nums2[j]) {
            curr = nums1[i];
            i += 1;
        } else {
            curr = nums2[j];
            j += 1;
        }
    }

    if total % 2 == 1 {
        curr as f64
    } else {
        (prev + curr) as f64 / 2.0
    }
}
}
