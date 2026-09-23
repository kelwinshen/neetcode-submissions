

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut array_freq_with_first_index = vec![(0, i32::MAX); 26];

        let mut index: i32 = 0;
        for ch in s.bytes(){
            array_freq_with_first_index[(ch-b'a') as usize].0+=1;
            if array_freq_with_first_index[(ch-b'a') as usize].1 == i32::MAX {
                 array_freq_with_first_index[(ch-b'a') as usize].1 = index;
            }
            index+=1;
        }

        let mut minimal_index: i32 = i32::MAX;

        for i in 0..array_freq_with_first_index.len(){
            if array_freq_with_first_index[i].0 == 1 {
              minimal_index =  minimal_index.min(array_freq_with_first_index[i].1);
            }
        }

        if minimal_index == i32::MAX {
            return -1;
        }

        minimal_index
    }
}
