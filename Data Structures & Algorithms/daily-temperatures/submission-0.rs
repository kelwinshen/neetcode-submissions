impl Solution {
  pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    // We initialize our result array with 0s. 
    // If a day never finds a warmer temperature, it will naturally just stay 0!
    let mut result: Vec<i32> = vec![0 as i32; temperatures.len()];
    
    // Your stack! Storing tuples: (temperature, index)
    let mut stack: Vec<(i32, usize)> = Vec::new();
    
    // We use .iter().enumerate() so we get both the index 'i' and the 'temp'
    for (i, &temp) in temperatures.iter().enumerate() {
        
        // Write your waiting room logic here!
        // Hint: Use a `while` loop that checks `stack.last()`
            
        while !stack.is_empty() && temp > stack.last().unwrap().0 {
            result[stack.last().unwrap().1] = (i - stack.last().unwrap().1) as i32;
            stack.pop();
        }

        stack.push((temp, i));

   
        
    }
    
    result
}
}
