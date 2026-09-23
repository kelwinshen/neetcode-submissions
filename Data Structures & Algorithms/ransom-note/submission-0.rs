impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut freq = [0; 256];

        for ch in magazine.bytes(){
            freq[(ch-b'a') as usize]+=1;
        }

        for ch in ransom_note.bytes(){
            if freq[(ch-b'a') as usize] > 0 {
               freq[(ch-b'a') as usize]-=1;
            } else{
                return false;
            }
        }

        true
    }
}
