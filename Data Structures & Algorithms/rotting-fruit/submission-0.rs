use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        //we can use bread first search to solve this: FIFO (First In First Out)
        let mut fresh_count = 0;

        let mut grid = grid;

        let mut stacks: VecDeque<(usize, usize)> = VecDeque::new();

        for r in 0..grid.len(){
            for c in 0..grid[0].len(){
                match grid[r][c]{
                    2=> stacks.push_back((r,c)),
                    1=> fresh_count+=1,
                    _=> {}
                }

            }
        }

        if fresh_count == 0 {
            return 0;
        }

        let directions: Vec<(i32, i32)> = vec![(1,0), (-1,0), (0,1), (0,-1)];

         
        let mut minutes = 0;

        while !stacks.is_empty() &&  fresh_count > 0{
            let level_size = stacks.len();
            minutes+=1;

            for _ in 0..level_size{
                if let Some((rotten_row, rotten_col)) = stacks.pop_front(){

                    for (dr, dc) in directions.iter(){
                        let potential_rotten_row = rotten_row as i32 + dr;
                        let potential_rotten_col = rotten_col as i32 + dc;


                if potential_rotten_row < 0 ||  potential_rotten_col < 0 || potential_rotten_row >= grid.len() as i32 ||  potential_rotten_col >= grid[0].len() as i32 {
                    continue;
                }

                if grid[potential_rotten_row as usize][potential_rotten_col as usize] == 1 {
                    stacks.push_back((potential_rotten_row as usize, potential_rotten_col as usize));
                    grid[potential_rotten_row as usize][potential_rotten_col as usize] = 2;
                    fresh_count-=1;
                }




                    }


                }
            }
        }


    if fresh_count == 0 {
       return minutes;
    } 

        -1
    }
}
