struct NumMatrix {
    matrix: Vec<Vec<i32>>,
    sum_vec: Vec<Vec<i32>>
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut sum_vec: Vec<Vec<i32>> = Vec::new();
        for i in 0..matrix.len(){
            let mut vec_row_sum: Vec<i32> = Vec::new();
            let mut current_sum = 0;
            for j in 0..matrix[0].len(){
               current_sum += matrix[i][j];
               vec_row_sum.push(current_sum);
            }

            sum_vec.push(vec_row_sum);

        }
        NumMatrix{
            matrix,
            sum_vec
        }
    }
    //2,1,4,3 -> 8
    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut sum = 0;
        for i in (row1 as usize)..=(row2 as usize){
            if col1 > 0 {
                  sum+=self.sum_vec[i][col2 as usize]-self.sum_vec[i][col1 as usize-1];
            } else {
                 sum+=self.sum_vec[i][col2 as usize]
            }
        
        }

        sum
        
    }
}
