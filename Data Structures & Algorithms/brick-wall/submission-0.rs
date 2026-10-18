impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut freq_map:  HashMap<i32, i32> = HashMap::new();
        for i in 0..wall.len(){
             let mut current = 0;
            for j in 0..wall[i].len()-1{
              current += wall[i][j];
                *freq_map.entry(current).or_insert(0) += 1;
            }
        }

        

      
        let mut max_freq: i32 = 0;

        println!("{:?}", freq_map);

        for (x ,freq) in freq_map.into_iter(){
           if freq > max_freq {
            max_freq = freq;
           }
        }

        wall.len() as i32 - max_freq
    }
}

// Row 0:  [ 1 ][   2   ][   2   ][ 1 ]   -> Edges at: x = 1, 3, 5
// Row 1:  [   3   ][ 1 ][   2   ]       -> Edges at: x = 3, 4
// Row 2:  [ 1 ][     3     ][   2   ]   -> Edges at: x = 1, 4
// Row 3:  [   2   ][       4       ]   -> Edges at: x = 2
// Row 4:  [ 1 ][ 3 ][   1   ][ 1 ]       -> Edges at: x = 1, 4, 5

//find freq the most that last 

