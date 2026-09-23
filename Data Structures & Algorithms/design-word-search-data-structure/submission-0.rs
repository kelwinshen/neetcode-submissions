struct TrieNode{
   children: [Option<Box<TrieNode>>; 26],
   is_the_end_of_the_word: bool
}

impl TrieNode{
    fn new()->Self{
       TrieNode{
         children: Default::default(),
        is_the_end_of_the_word: false
       }
    }
}

struct WordDictionary {
    root: TrieNode
}

impl WordDictionary {
    pub fn new() -> Self {
        WordDictionary{
            root: TrieNode::new()
        }
    }

    pub fn add_word(&mut self, word: String) {
        let mut node = &mut self.root;

        for ch in word.bytes(){
            let index = (ch-b'a') as usize;
            node = node.children[index].get_or_insert_with(|| Box::new(TrieNode::new()));
        }

        node.is_the_end_of_the_word = true;

    }

   fn search(&self, word: String) -> bool {
        let chars: Vec<u8> = word.bytes().collect();
        Self::backtrack(&self.root, &chars, 0)
    }

    fn backtrack(node: &TrieNode, chars: &Vec<u8>, index: usize) -> bool {
        if index == chars.len() {
            return node.is_the_end_of_the_word;
        }

        let ch = chars[index];

        if ch == b'.' {
            // wildcard: try EVERY possible child
            for child_opt in node.children.iter() {
                if let Some(child) = child_opt {
                    if Self::backtrack(child, chars, index + 1) {
                        return true; // short-circuit, same as Word Search's || chain
                    }
                }
            }
            false
        } else {
            // normal letter: only one specific child to check
            let idx = (ch - b'a') as usize;
            match &node.children[idx] {
                Some(child) => Self::backtrack(child, chars, index + 1),
                None => false,
            }
        }
    }
}
