impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut current: String = String::new();
        Self::backtrack(0, 0, n, &mut current, &mut result);
        result
    }

    fn backtrack(open_count: i32, close_count: i32, n: i32, current: &mut String, result: &mut Vec<String>) {
        if current.len() == 2 * n as usize {
            result.push(current.clone());
            return;
        }

        if open_count < n {
            current.push('(');
            Self::backtrack(open_count + 1, close_count, n, current, result);
            current.pop();
        }

        if close_count < open_count {
            current.push(')');
            Self::backtrack(open_count, close_count + 1, n, current, result);
            current.pop();
        }
    }
}