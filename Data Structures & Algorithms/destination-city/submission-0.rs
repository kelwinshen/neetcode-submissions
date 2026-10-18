impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        //a = b
        //c = a
        // ----- +
        // c = b 
        // b is Oak 

        //a = A
        //A = Z
        // ---- +
        // a = Z
        // Z 

        let mut starts = HashSet::new();

        // Step 1: Collect all starting cities
        for path in &paths {
            starts.insert(&path[0]);
        }

        // Step 2: Find the destination city that isn't a starting city
        for path in &paths {
            let dest = &path[1];
            if !starts.contains(dest) {
                return dest.clone();
            }
        }

        String::new()
    }
}
