use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

// ==========================================
// 1. THE DATA STRUCTURES
// ==========================================

// This is just a shortcut. Instead of typing `Option<Rc<RefCell<Node>>>` everywhere, 
// we can just type `Link`. It represents a shared, mutable pointer to a Node.
type Link = Option<Rc<RefCell<Node>>>;

// Think of a Node as a single "Book" on your desk.
struct Node {
    // WHY DO WE NEED THE KEY?
    // If the desk is full, we throw away the bottom book. 
    // We must look at its `key` so we can tell the HashMap to forget it too!
    key: i32,   
    value: i32, 
    
    // The book sitting below this one
    prev: Link, 
    // The book sitting above this one
    next: Link, 
}

impl Node {
    // A helper to create a brand new Book and immediately wrap it in our 
    // Rust smart pointers (Rc for shared ownership, RefCell for mutability).
    fn new(key: i32, value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            key,
            value,
            prev: None,
            next: None,
        }))
    }
}

pub struct LRUCache {
    capacity: usize, // How many books fit on the desk
    
    // THE BRAIN: Instantly finds a book in O(1) time. 
    // Key = Book Title, Value = Shared pointer to the physical Book.
    map: HashMap<i32, Rc<RefCell<Node>>>,
    
    // THE BOOKENDS: 
    // 'head' is a fake book that always sits at the absolute TOP of the pile.
    // 'tail' is a fake book that always sits at the absolute BOTTOM of the pile.
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        // Create our unbreakable "Dummy" bookends. 
        // We give them key=0, value=0 because their data doesn't matter.
        let head = Node::new(0, 0);
        let tail = Node::new(0, 0);

        // Immediately link them together: [HEAD] <---> [TAIL]
        // Now, every real book we insert will be sandwiched securely between them!
        head.borrow_mut().next = Some(Rc::clone(&tail));
        tail.borrow_mut().prev = Some(Rc::clone(&head));

        LRUCache {
            capacity: capacity as usize,
            map: HashMap::new(),
            head,
            tail,
        }
    }

    // ==========================================
    // 2. THE ROBOT ARMS (HELPER FUNCTIONS)
    // ==========================================

    // HELPER 1: Yanks a book out of the middle of the pile.
    // Time Complexity: O(1)
    fn remove_node(&mut self, node: Rc<RefCell<Node>>) {
        // Look at the book we want to remove. 
        // Find the book 'above' it (prev) and the book 'below' it (next).
        let prev_node = Rc::clone(node.borrow().prev.as_ref().unwrap());
        let next_node = Rc::clone(node.borrow().next.as_ref().unwrap());

        // Glue the book above directly to the book below!
        // This causes the current 'node' to be snipped completely out of the chain.
        prev_node.borrow_mut().next = Some(Rc::clone(&next_node));
        next_node.borrow_mut().prev = Some(Rc::clone(&prev_node));
    }

    // HELPER 2: Drops a book at the very top of the pile (Most Recently Used).
    // Time Complexity: O(1)
    fn insert_at_head(&mut self, node: Rc<RefCell<Node>>) {
        // The Dummy Head is always the top bookend.
        // Find the "real" book currently sitting just below the Dummy Head.
        let next_node = Rc::clone(self.head.borrow().next.as_ref().unwrap());

        // Step 1: Tell our NEW book to point UP to the Dummy Head, and DOWN to the old top book.
        node.borrow_mut().prev = Some(Rc::clone(&self.head));
        node.borrow_mut().next = Some(Rc::clone(&next_node));

        // Step 2: Tell the Dummy Head and the old top book to point to our NEW book.
        self.head.borrow_mut().next = Some(Rc::clone(&node));
        next_node.borrow_mut().prev = Some(Rc::clone(&node));
    }

    // ==========================================
    // 3. MAIN LOGIC: GET & PUT
    // ==========================================

    // Goal: Read a book, and immediately move it to the top of the pile.
    pub fn get(&mut self, key: i32) -> i32 {
        // Ask the Brain (HashMap): "Do we have this book?"
        if let Some(node) = self.map.get(&key) {
            
            // We found it! We must clone the pointer so we don't accidentally 
            // remove the original pointer from the HashMap.
            let node_clone = Rc::clone(node);
            
            // Yank it out of wherever it is currently sitting...
            self.remove_node(Rc::clone(&node_clone));
            
            // ...and drop it at the very top! (It is now the Most Recently Used)
            self.insert_at_head(Rc::clone(&node_clone));
            
            // Open the book and return the value to the user.
            return node_clone.borrow().value;
        }
        
        -1 // The Brain says: "We don't have that book."
    }

    // Goal: Add a new book, or update an existing one. Kick out the oldest if full!
    pub fn put(&mut self, key: i32, value: i32) {
        
        // Ask the Brain (HashMap): "Do we already have a book with this title?"
        if let Some(node) = self.map.get(&key) {
            
            // SCENARIO 1: The book already exists!
            let node_clone = Rc::clone(node);
            
            // Update the text inside the book
            node_clone.borrow_mut().value = value;
            
            // Because we just touched it, yank it out and move it to the top!
            self.remove_node(Rc::clone(&node_clone));
            self.insert_at_head(Rc::clone(&node_clone));
            
        } else {
            // SCENARIO 2: It is a brand new book!
            
            // Create the new physical book in memory
            let new_node = Node::new(key, value);
            
            // Tell the Brain exactly where this new book lives
            self.map.insert(key, Rc::clone(&new_node));
            
            // Drop it at the very top of our pile
            self.insert_at_head(Rc::clone(&new_node));

            // SCENARIO 3: THE EVICTION (Are there too many books on the desk?)
            if self.map.len() > self.capacity {
                
                // Find the book sitting at the absolute bottom of the pile.
                // It is the one sitting right 'above' (prev) our Dummy Tail bookend.
                let lru_node = Rc::clone(self.tail.borrow().prev.as_ref().unwrap());
                
                // Tell the Brain to completely forget about this book's title!
                // (This is why Node has a `key` field: so we know what to tell the HashMap)
                self.map.remove(&lru_node.borrow().key);
                
                // Yank the book out of the pile and throw it in the trash.
                self.remove_node(lru_node);
            }
        }
    }
}