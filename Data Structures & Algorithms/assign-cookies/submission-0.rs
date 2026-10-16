impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort(); // least greedy child first
        s.sort(); // smallest cookie first

        let mut child = 0; // the least greedy child who is still sad

        for cookie in s {
            // this cookie is big enough for that child? then he is happy, move to the next child
            if child < g.len() && cookie >= g[child] {
                child += 1;
            }
            // otherwise the cookie is too small for EVERY remaining child, so we just skip it
        }

        child as i32
    }
}