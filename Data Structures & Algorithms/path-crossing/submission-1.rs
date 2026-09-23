impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let directions: Vec<(i32, i32)> = vec![(1,0), (-1,0), (0,-1), (0,1)];
        let mut origins: (i32, i32) = (0,0);
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        visited.insert((0,0));
        for ch in path.chars(){
            if ch == 'N' {
                origins.0 += directions[0].0;
                  origins.1 += directions[0].1;
            } else if ch == 'S' {
                origins.0 += directions[1].0;
                  origins.1 += directions[1].1;
            } else if ch == 'E' {
                origins.0 += directions[2].0;
                  origins.1 += directions[2].1;
            } else {
                 origins.0 += directions[3].0;
                  origins.1 += directions[3].1;
            }

             if visited.contains(&origins){
                return true;
            }
            
            visited.insert(origins);

        }   

        false
    }
}