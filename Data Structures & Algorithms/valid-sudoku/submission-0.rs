use std::collections::HashMap;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        //Check column and row freq no more than 1 at first 

        for i in 0..board.len(){
            let mut freq_hashmap: HashMap<char, i32> = HashMap::new();
            for j in 0..board.len(){

           *freq_hashmap.entry(board[i][j]).or_insert(0) += 1;
               
            if freq_hashmap.get(&board[i][j]).unwrap() > &1 && &board[i][j] != &'.' {
                println!("{}", freq_hashmap.get(&board[i][j]).unwrap());
                return false;
            }
            }
           
        }

        //[1,2,3], [4,5,6], [7,8,9]
        //[1,4,7], [2,5,8], [3,6,9]


        for i in 0..board.len(){
            let mut freq_hashmap_col: HashMap<char, i32> = HashMap::new();
           let mut current_column: Vec<char> = Vec::new();

            for j in 0..board.len(){
                current_column.push(board[j][i]);
            }

            for k in 0..board.len(){
                 *freq_hashmap_col.entry(current_column[k]).or_insert(0) += 1;


            if freq_hashmap_col.get(&current_column[k]).unwrap() > &1 && current_column[k] != '.' {
                return false;
            }
            }

        
        }

        // [1,2,3], [4,5,6] , [7,8,9]
        // [1,2,3], [4,5,6] , [7,8,9]
        // [1,2,3], [4,5,6] , [7,8,9]
        
        //Check sub-boxes by 
       

        // 1. Iterate over the 3x3 "Macro Blocks"
for block_r in 0..3 {
    for block_c in 0..3 {
        
        // We are now inside ONE specific 3x3 box. 
        // Create your hashmap for this box!
        let mut freq_hashmap_box: HashMap<char, i32> = HashMap::new();

        // 2. Iterate over the 3x3 "Micro Cells" inside this specific block
        for i in 0..3 {
            for j in 0..3 {
                // THE MAGIC FORMULA:
                let exact_row = (block_r * 3) + i;
                let exact_col = (block_c * 3) + j;
                
                let current_char = board[exact_row][exact_col];

                // Now use your exact same frequency logic!
                if current_char != '.' {
                    *freq_hashmap_box.entry(current_char).or_insert(0) += 1;

                    if freq_hashmap_box.get(&current_char).unwrap() > &1 {
                        return false; // Found a duplicate in this 3x3 box!
                    }
                }
            }
        }
        
    }
}


        

        

           true

    }
}
