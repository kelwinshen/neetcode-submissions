use std::collections::HashSet;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut cols: HashSet<i32> = HashSet::new();
        let mut diag1: HashSet<i32> = HashSet::new(); // row - col
        let mut diag2: HashSet<i32> = HashSet::new(); // row + col
        let mut queen_cols: Vec<usize> = Vec::new(); // queen_cols[row] = which column the queen is in
        let mut result: Vec<Vec<String>> = Vec::new();

        Self::backtrack(0, n, &mut cols, &mut diag1, &mut diag2, &mut queen_cols, &mut result);
        result
    }

    fn backtrack(
        row: usize,
        n: usize,
        cols: &mut HashSet<i32>,
        diag1: &mut HashSet<i32>,
        diag2: &mut HashSet<i32>,
        queen_cols: &mut Vec<usize>,
        result: &mut Vec<Vec<String>>,
    ) {
        if row == n {
            result.push(Self::build_board(queen_cols, n));
            return;
        }

        for col in 0..n {
            let c = col as i32;
            let r = row as i32;
            if cols.contains(&c) || diag1.contains(&(r - c)) || diag2.contains(&(r + c)) {
                continue; // illegal: this column, or one of its diagonals, is already attacked
            }

            // choose
            cols.insert(c);
            diag1.insert(r - c);
            diag2.insert(r + c);
            queen_cols.push(col);

            // explore
            Self::backtrack(row + 1, n, cols, diag1, diag2, queen_cols, result);

            // un-choose
            cols.remove(&c);
            diag1.remove(&(r - c));
            diag2.remove(&(r + c));
            queen_cols.pop();
        }
    }

    fn build_board(queen_cols: &Vec<usize>, n: usize) -> Vec<String> {
        queen_cols
            .iter()
            .map(|&col| {
                let mut row_str = String::new();
                for c in 0..n {
                    row_str.push(if c == col { 'Q' } else { '.' });
                }
                row_str
            })
            .collect()
    }
}