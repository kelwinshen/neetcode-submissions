impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        
        let mut answers: Vec<i32> = Vec::new();

        for i in 0..nums1.len(){
            'scope: for j in 0..nums2.len(){
               
                if nums1[i] == nums2[j]{
                    for k in j..nums2.len(){
                        if nums2[k] > nums2[j] {
                           answers.push(nums2[k]);
                           break 'scope;
                        }
                    }

                      answers.push(-1);
                }
            }
        }   

        answers
      
    }
}
