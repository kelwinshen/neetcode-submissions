impl Solution {
   fn search_matrix( matrix: Vec<Vec<i32>>, target: i32)->bool{
    //matrix structure is 
    //[row1, row2, row3, ....]
    //we need to find our target in which row based on the information 
    // Each row in matrix is sorted in non-decreasing order.
    // The first integer of every row is greater than the last integer of the previous row.
    
    
    let mut target_row_index = 0;
    
    for i in 0..matrix.len(){
       if  matrix[i][0] > target {
           break;
       }
       target_row_index = i;
    }
    
    let target_row = &matrix[target_row_index];
    
    let mut left = 0;
    let mut right = target_row.len();
    
    
    while left < right {
        let middle = left + (right-left)/2;
        
        if target_row[middle] == target {
            return true;
        } else if target_row[middle] > target {
            right = middle;
        } else {
            left = middle + 1;
        }
    }
    
    
    false
    
    
    
}

}
