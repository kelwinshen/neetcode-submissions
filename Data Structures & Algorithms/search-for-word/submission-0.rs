impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word_chars: Vec<char> = word.chars().collect();
        let rows = board.len();
        let cols = board[0].len();
        let mut board = board; // need mutable to mark visited in-place

        for r in 0..rows {
            for c in 0..cols {
                if Self::backtrack(&mut board, &word_chars, r, c, 0) {
                    return true; // found a valid starting point — stop immediately
                }
            }
        }
        false
    }

    fn backtrack(
        board: &mut Vec<Vec<char>>,
        word: &Vec<char>,
        r: usize,
        c: usize,
        index: usize,
    ) -> bool {
        // Base case: matched every character
        if index == word.len() {
            return true;
        }
        // Out of bounds, or this cell doesn't match the letter we need
        if r >= board.len() || c >= board[0].len() || board[r][c] != word[index] {
            return false;
        }

        // choose: mark this cell as visited (temporarily overwrite it)
        let original = board[r][c];
        board[r][c] = '#'; // sentinel value that can't match any real letter

        // explore: try all 4 directions
        let found = 
            (r > 0 && Self::backtrack(board, word, r - 1, c, index + 1)) ||
            Self::backtrack(board, word, r + 1, c, index + 1) ||
            (c > 0 && Self::backtrack(board, word, r, c - 1, index + 1)) ||
            Self::backtrack(board, word, r, c + 1, index + 1);

        // un-choose: restore the original letter
        board[r][c] = original;

        found
    }
}