impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        //we figure out the left time car need to travel

        let mut time_need_to_finish: Vec<(f64, i32)> = Vec::new();

        //define the time needed
        //(target-position[i])/speed

        for i in 0..speed.len(){
            let time = ((target - position[i])as f64)/speed[i] as f64;
            time_need_to_finish.push((time, position[i]));
        }

        time_need_to_finish.sort_by(|a,b| b.1.cmp(&a.1)); //sort based on the position 

        let mut fleets: Vec<f64> = Vec::new();

        for time in time_need_to_finish {
            if let Some(fleet) = fleets.last() {
                if fleet < &time.0 {
                    fleets.push(time.0);
                }
            } else{
                 fleets.push(time.0);
            }
        }

        fleets.len() as i32







        
    }
}
