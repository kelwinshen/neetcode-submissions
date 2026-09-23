
impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut sorted_vec_height: Vec<(i32, usize)> = Vec::new();
        for i in 0..heights.len(){
            sorted_vec_height.push((heights[i], i));
        }

        sorted_vec_height.sort_by(|a,b| b.0.cmp(&a.0));

        let mut answer: Vec<String> = Vec::new();

        for i in 0..sorted_vec_height.len(){
          answer.push(names[sorted_vec_height[i].1].clone());
        }

        answer


    }
}