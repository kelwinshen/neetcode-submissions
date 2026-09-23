impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        
        // ❌ YOUR OLD CODE: let mut k = piles.iter().max().unwrap().clone();
        // ✅ FIX 1: Set up proper boundaries instead of just starting at max.
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();

        // ❌ YOUR OLD CODE: loop {
        // ✅ FIX 2: Use a while loop to keep shrinking the window until left == right.
        while left < right {
            
            // In Binary Search, 'k' (our speed) is always the middle of our boundaries!
            let k = left + (right - left) / 2;
            
            let mut h_total: i64 = 0; // Changed to i64 to prevent overflow crashes!

            for i in 0..piles.len() {
                // ❌ YOUR OLD CODE: 
                // let mut h_pile = 1;
                // while k * h_pile < piles[i] { h_pile+=1; }
                
                // ✅ FIX 3: Replace the slow while loop with the O(1) math trick!
                let h_pile = ((piles[i] - 1) / k) + 1;
                
                h_total += h_pile as i64;
            }

            // ❌ YOUR OLD CODE: 
            // if h_total > h { break; }
            // k = k / 2;
            
            // ✅ FIX 4: Update boundaries instead of breaking!
            if h_total <= h as i64 {
                // Koko finished on time! Let's see if she can do it even slower.
                right = k; 
            } else {
                // Koko was too slow (h_total > h). She MUST eat faster!
                left = k + 1; 
            }
        }

        // When left and right meet, we found the perfect minimum speed.
        left
    }
}