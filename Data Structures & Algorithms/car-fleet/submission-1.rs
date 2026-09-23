use std::cmp;
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
 

       let mut more_time_to_finish_vec: Vec<(f32, i32)> = Vec::new();


       for i in 0..position.len(){
        more_time_to_finish_vec.push( ((target as f32 - position[i] as f32)/speed[i] as f32, position[i]) );
       }

       more_time_to_finish_vec.sort_by(|a,b| {
        b.1.cmp(&a.1)
       }); 

       let mut stacks: Vec<f32> = Vec::new();

    
        for time in more_time_to_finish_vec {
            if let Some(a) =  stacks.last()  {
               if a < &time.0 {
                stacks.push(time.0);
               }
                  
            } else {
                 stacks.push(time.0);
            }
          
          
        }

stacks.len() as i32













    }
}
