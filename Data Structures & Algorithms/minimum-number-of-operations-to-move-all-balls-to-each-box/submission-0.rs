impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let boxes_vec: Vec<i32> = boxes.chars().into_iter().map(|a|a.to_digit(10).unwrap() as i32 ).collect();
    
        let mut result: Vec<i32> = Vec::new();

        let mut boxes_with_ball: Vec<i32> = Vec::new();

        for i in 0..boxes_vec.len(){
            if boxes_vec[i] == 1{
                  boxes_with_ball.push(i as i32);
            }
        }

         for i in 0..boxes_vec.len(){
            let mut current = 0;
            for j in 0..boxes_with_ball.len(){
                current += (boxes_with_ball[j]-i as i32).abs();
            }

            result.push(current);
            
        }

        result





    }
}



