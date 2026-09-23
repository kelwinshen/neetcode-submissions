impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>)->i32{
    let mut grid = grid;
    
    let mut max = 0;
    
    for r in 0..grid.len(){
        for c in 0..grid[0].len(){
            let mut count = 0;
            if grid[r][c] == 1 {
                Self::floots_and_max(&mut grid, r, c, &mut count);
               max = max.max(count);
            }
            max = max.max(count);
        }
    }
    
    max
}

fn floots_and_max(grid: &mut Vec<Vec<i32>>, r: usize, c: usize, count: &mut i32){
    if r >= grid.len() || c >= grid[0].len() ||  grid[r][c] != 1 {
        return;
    }
    
    *count+=1;
    
    grid[r][c] = 0;
    
    if r > 0 {
        Self::floots_and_max(grid, r-1, c, count);
    }
    
     Self::floots_and_max(grid, r+1, c, count);
     
      if c > 0 {
        Self::floots_and_max(grid, r, c-1, count);
    }
    
     Self::floots_and_max(grid, r, c+1, count);
}

}
