impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        //permutation means combinations of those character with positions matters

        //we can start check in range of element between start until start + s1.len() across
        //s2, if we found that freq vector of s1 is match with what we found in the s2 when 
        //we check then we can return true, otherwise we return false


        let mut freq_s1 = [0; 26];
        for ch in s1.bytes(){
             freq_s1[(ch-b'a') as usize] +=1;
        }

        let mut start: usize = 0;

        let mut s2_bytes: Vec<u8> = s2.bytes().collect();


        while start + s1.len() <= s2.len() {
            let mut freq_s2= [0; 26];

            for i in start..start+s1.len(){

                freq_s2[(s2_bytes[i]-b'a') as usize] +=1;
            }

            if freq_s2 == freq_s1 {
                return true;
            }

            start+=1;



        }

        return false;

    }
}
