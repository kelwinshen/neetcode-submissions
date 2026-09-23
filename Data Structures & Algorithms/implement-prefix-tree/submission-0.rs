
struct TrieNode {
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

struct PrefixTree{
    root: TrieNode
}

impl PrefixTree {
    pub fn new() -> Self {
        PrefixTree{
            root: TrieNode::new()
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for ch in word.bytes(){
            let index = (ch-b'a') as usize;
            node = node.children[index].get_or_insert(Box::new(TrieNode::new()));
        }

        node.is_the_end_of_the_word = true;

    }

    pub fn search(&self, word: String) -> bool {
        let mut node = & self.root;

        for ch in word.bytes() {
        let index = (ch - b'a') as usize;

        match &node.children[index] {
            Some(next) => node = next,
            None => return false, // path doesn't exist — stop immediately
        }
    }

        node.is_the_end_of_the_word 
    }

    pub fn starts_with(&self, prefix: String) -> bool {
         let mut node = & self.root;

        for ch in prefix.bytes() {
        let index = (ch - b'a') as usize;

        match &node.children[index] {
            Some(next) => node = next,
            None => return false, // path doesn't start with that prefix
        }
    }

        true
    }
}
