impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        //we can solve this using deepth first search
        
        let mut grid = grid;
        let mut count = 0;

        for r in 0..grid.len(){
            for c in 0..grid[0].len(){
                if grid[r][c] == '1' {
                    count+=1;
                    Self::floods(&mut grid, c, r);
                }
            }
        }

        count 



    }

    pub fn floods(grid: &mut  Vec<Vec<char>>, c: usize, r: usize){
        if c == grid[0].len() || r == grid.len() || grid[r][c] != '1' {
            return;
        }

       grid[r][c] = '0';

       if r > 0 {
        Self::floods(grid, c, r-1);
       }

         Self::floods(grid, c, r+1);

          if c > 0 {
        Self::floods(grid, c-1, r);
       }

     Self::floods(grid, c+1, r);
    }
}
