impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut total_delay: i64 = 0;
        let mut time_tick: i64 = 0;
        let mut amount_delay: i64 = 0;
       
        for i in 0..customers.len(){
           let arrival = customers[i][0].clone() as i64;
           
           if time_tick < arrival {
                time_tick = arrival;
           }

            let time = customers[i][1].clone() as i64; 
           
              time_tick += time;
                total_delay+= (time_tick-arrival);

        }

        total_delay as f64/customers.len() as f64
    }
}