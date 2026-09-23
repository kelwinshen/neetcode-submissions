impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left: i32 = 1;
        let mut right: i32 = piles.iter().max().unwrap().clone();
        

        while left < right {
            let mut h_total = 0;
            let k = left + (right-left)/2;

            for i in 0..piles.len(){
               let h_pile = (piles[i]-1)/k + 1;
                  h_total+= h_pile;
            }

            if h_total > h {
                 left = k + 1;
            } else {
                  right = k;
            }
        }


        left
    }
}