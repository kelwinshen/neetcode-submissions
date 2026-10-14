impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
               let length = s.len();

               if length == 1 {
                return true;
               }
       
       if Self::is_palindrome(&s) {
            return true;
       }


       for i in 0..length{
            let mut new_input = s.clone();
            new_input.remove(i);

            if Self::is_palindrome(&new_input){
                return true;
            }

       }

       return false;
    }

    pub fn is_palindrome(input: &String)->bool {
        let length = input.len();
          if length == 1 {
                return true;
               }

         let mut left = 0;
        let mut right = input.len()-1;

        while left <= right {
           if input[left..=left] != input[right..=right] {
                return false;
           }
           left+=1;
           right-=1;
        }

        return true;
    }
}
