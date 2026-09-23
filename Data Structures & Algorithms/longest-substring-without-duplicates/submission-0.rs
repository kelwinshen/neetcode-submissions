impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();

        let mut duplicate_check_array = [false; 128]; 

        let mut left_pointer: usize = 0;
       
       
        let mut max = 0;

        for right_pointer in 0..bytes.len(){
            if  duplicate_check_array[bytes[right_pointer] as usize] == true {
               
                while left_pointer < right_pointer{
                     if bytes[right_pointer] ==  bytes[left_pointer] {
                        left_pointer+=1;
                        break;
              } 
                duplicate_check_array[bytes[left_pointer] as usize] =  false;
               left_pointer+=1;
                }
             
            }

            duplicate_check_array[bytes[right_pointer] as usize] = true;

            if max < right_pointer - left_pointer + 1 {
                max = right_pointer - left_pointer + 1;
            }

          

           
        }

        

        

    max as i32

    }
}
