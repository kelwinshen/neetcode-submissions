struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    word: Option<String>, // storing the actual word here avoids rebuilding the string during DFS
}

impl TrieNode {
    fn new() -> Self {
        TrieNode { children: Default::default(), word: None }
    }
}

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        // Step 1: build the Trie from all words
        let mut root = TrieNode::new();
        for word in &words {
            let mut node = &mut root;
            for ch in word.bytes() {
                let idx = (ch - b'a') as usize;
                node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
            }
            node.word = Some(word.clone());
        }

        let mut result: Vec<String> = Vec::new();
        let rows = board.len();
        let cols = board[0].len();

        // Step 2: DFS from every cell, walking the Trie alongside the board
        for r in 0..rows {
            for c in 0..cols {
                Self::backtrack(&mut board, &mut root, r, c, &mut result);
            }
        }

        result
    }

    fn backtrack(
        board: &mut Vec<Vec<char>>,
        node: &mut TrieNode,
        r: usize,
        c: usize,
        result: &mut Vec<String>,
    ) {
        let ch = board[r][c];
        if ch == '#' {
            return; // already used in this path
        }
        let idx = (ch as u8 - b'a') as usize;

        // does the Trie even have a path for this letter?
        let child = match &mut node.children[idx] {
            Some(child) => child,
            None => return,
        };

        // found a complete word — record it, and clear it so we don't add it twice
        if let Some(word) = child.word.take() {
            result.push(word);
        }

        // mark visited, explore all 4 directions
        board[r][c] = '#';
        if r > 0 { Self::backtrack(board, child, r - 1, c, result); }
        if r + 1 < board.len() { Self::backtrack(board, child, r + 1, c, result); }
        if c > 0 { Self::backtrack(board, child, r, c - 1, result); }
        if c + 1 < board[0].len() { Self::backtrack(board, child, r, c + 1, result); }
        board[r][c] = ch; // un-choose
    }
}