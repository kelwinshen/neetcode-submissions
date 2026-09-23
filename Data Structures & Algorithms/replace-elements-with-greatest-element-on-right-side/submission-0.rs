impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {

        for i in 0..arr.len(){
           let mut max = i32::MIN;
           for j in (i+1)..arr.len(){
                if arr[j] > max {
                    max = arr[j];
                }
           }

           arr[i] = max;

          

           if i == arr.len() - 1 {
             arr[i] = -1;
           }
        }

        
arr
    }
}
