use std::collections::VecDeque;

impl Solution {
    pub fn time_required_to_buy(mut tickets: Vec<i32>, k: i32) -> i32 {
        //number of tickets of length n where number of tickets want to buy
        let mut times = 0;

        'scope: loop {
            for i in 0..tickets.len(){
                if tickets[i] > 0 {
                     times+=1;
                      tickets[i] -=1;
                }
               

                if tickets[k as usize] == 0 {
                    break 'scope;
                }

            }
        }

        times

    }
}