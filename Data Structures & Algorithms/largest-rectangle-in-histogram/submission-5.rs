impl Solution {
   pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut max_area = 0;
    
    // The Stack acts as our "Waiting Room" for rectangles.
    // It stores a tuple: (index_it_started_at, height)
    let mut stack: Vec<(usize, i32)> = Vec::new(); 

    for (i, &current_height) in heights.iter().enumerate() {
        // By default, a bar's rectangle starts at its own index.
        let mut start_index = i;

        // THE COLLISION: If the current bar is SHORTER than the top of our stack, 
        // the top of the stack has hit a wall! It CANNOT expand any further right.
        while let Some(&(prev_index, prev_height)) = stack.last() {
            if prev_height > current_height {
                // 1. Pop the tall bar because its journey is over.
                stack.pop();
                
                // 2. Calculate its final area: height * (current_index - start_index)
                let width = (i - prev_index) as i32;
                let area = prev_height * width;
                if area > max_area {
                    max_area = area;
                }
                
                // 3. ⭐ THE MAGIC TRICK ⭐
                // Because the popped bar was TALLER than our current short bar,
                // our short bar can actually slide backwards into the space the 
                // tall bar used to occupy! We steal its starting index.
                start_index = prev_index;
            } else {
                break;
            }
        }

        // Push the current bar into the stack, using its newly extended start index!
        stack.push((start_index, current_height));
    }

    // THE CLEANUP: Any bars still left in the stack never hit a shorter wall.
    // This means they successfully extended all the way to the end of the array!
    for (start_index, height) in stack {
        let width = (heights.len() - start_index) as i32;
        let area = height * width;
        if area > max_area {
            max_area = area;
        }
    }

    max_area
}
}
