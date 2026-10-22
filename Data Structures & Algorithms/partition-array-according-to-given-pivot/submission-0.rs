impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut less = Vec::new();
    let mut equal = Vec::new();
    let mut greater = Vec::new();

    for &x in &nums {
        if x < pivot { less.push(x); }
        else if x == pivot { equal.push(x); }
        else { greater.push(x); }
    }

    less.extend(equal);
    less.extend(greater);
    less
}
}
