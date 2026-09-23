impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut bytes = s.to_lowercase().into_bytes();

        let mut write: usize = 0;
        for read in 0..bytes.len(){
            if bytes[read].is_ascii_alphanumeric(){
                bytes[write] = bytes[read];
                write+=1;
            }
        }

        bytes.truncate(write);


        for i in 0..bytes.len()/2{
           if bytes[i] != bytes[bytes.len()-1-i] {
            return false;
           }
        }

        true


      
   


    }
}
