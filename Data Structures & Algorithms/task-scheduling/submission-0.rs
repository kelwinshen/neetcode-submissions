use std::collections::{BinaryHeap, VecDeque};

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        // Step 1: count frequency of each task using a fixed 26-slot array
        let mut freq_task = [0; 26];
        for task in tasks {
            freq_task[(task as u8 - b'A') as usize] += 1;
        }

        // Step 2: push all non-zero counts into a max-heap
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        for freq in freq_task {
            if freq > 0 {
                heap.push(freq);
            }
        }

        // Step 3: cooldown queue holds (remaining_count, time_when_eligible_to_rejoin)
        let mut cooldown: VecDeque<(i32, i32)> = VecDeque::new();

        let mut time = 0;

        while !heap.is_empty() || !cooldown.is_empty() {
            time += 1;

            // Run the most frequent available task, if any
            if let Some(mut count) = heap.pop() {
                count -= 1;
                if count > 0 {
                    cooldown.push_back((count, time + n));
                }
            }
            // else: nothing available this cycle, it's idle — just let time tick

            // Check if the task at the front of cooldown is now eligible to rejoin
            if let Some(&(count, ready_time)) = cooldown.front() {
                if ready_time == time {
                    heap.push(count);
                    cooldown.pop_front();
                }
            }
        }

        time
    }
}