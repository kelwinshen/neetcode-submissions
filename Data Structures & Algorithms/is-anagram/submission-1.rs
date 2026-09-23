impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        
        //create two fixed array that store each of character of the string but converted into bytes that can be represent to the array index
        //the array value it self show the frequent of the specifc character occur or exist in the string 

        //we loop through both string to get array being filled

        //we compare if both same we return true and false otherwise

        if s.len() != t.len() {
            return false;
        }


        let mut s_vec: [i32; 26] = [0; 26];
         let mut t_vec: [i32; 26] = [0; 26];

         for ch in s.bytes(){
            s_vec[(ch-b'a') as usize] += 1;
         }

          for ch in t.bytes(){
            t_vec[(ch-b'a') as usize] += 1;
         }

         s_vec == t_vec



    }
}
