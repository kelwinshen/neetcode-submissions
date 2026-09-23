impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        //check the first column first using binary search to know whcih row the data will be
        //then after that binary search for that column 

          let mut first_column: Vec<(i32,usize)> = Vec::new();
        for i in 0..matrix.len(){
              first_column.push((matrix[i][0], i));
        }

        println!("{:?}", first_column);


        let mut target_row_index: usize = 0;
   
        for i in 0..first_column.len(){
          if target < first_column[i].0 {
            break;
          }
          target_row_index = i;
          }

     

        let target_row = &matrix[target_row_index];

            println!("{:?}", target_row);

        let mut start_pointer = 0;
        let mut end_pointer = target_row.len();

        while start_pointer < end_pointer {
            let middle_point = start_pointer + (end_pointer-start_pointer)/2;

            if target == target_row[middle_point] {
                return true;
            } else if target_row[middle_point] > target {
                end_pointer = middle_point;
            } else {
                start_pointer = middle_point + 1;
            }
        }



    false

      
    }
}
