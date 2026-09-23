impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        //nums1: [1,3] -> max 3, min 1
        //nums2: [2,4] -> max 4, min 2
        //merge together we have 1,3,2,4
        //[1,2,3,4]
        //2 + 3 /2 
        //[1,2,3]
        //2
        

        //as long as cut of sorter 

        nums1.extend(&nums2);
        nums1.sort();

        if nums1.len() % 2 == 0 {
            return (nums1[nums1.len()/2] + nums1[nums1.len()/2-1]) as f64 /2 as f64
        }

        nums1[nums1.len()/2] as f64
 


    }
}
