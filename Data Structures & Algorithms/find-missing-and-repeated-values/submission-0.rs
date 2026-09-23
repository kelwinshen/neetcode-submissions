use std::collections::HashMap;

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut grid_exist: Vec<bool> = vec![false; grid.len() * grid.len()];

        let mut answer: Vec<i32> = Vec::new();

        let mut hashmap_freq: HashMap<i32, i32> = HashMap::new();

        for row_index in 0..grid.len(){
            for column_index in 0..grid[0].len(){
                *hashmap_freq.entry(grid[row_index][column_index]).or_insert(0) += 1;

                if hashmap_freq.get(&grid[row_index][column_index]).unwrap() > &1 {
                answer.push(grid[row_index][column_index].clone());
            }

              grid_exist[grid[row_index][column_index] as usize - 1] = true;
            }

            


          


        }

        for i in 0..grid_exist.len(){
            if grid_exist[i] == false {
                answer.push(i as i32 + 1);
            }
        }

        answer


    }

   
}
