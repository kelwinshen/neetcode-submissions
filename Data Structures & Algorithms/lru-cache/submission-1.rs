use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashMap;

struct Node {
    key: i32,
    val: i32,
    prev: Option<Weak<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { key, val, prev: None, next: None }))
    }
}

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, Rc<RefCell<Node>>>,
    head: Rc<RefCell<Node>>, // dummy sentinel: "most recently used" side
    tail: Rc<RefCell<Node>>, // dummy sentinel: "least recently used" side
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let head = Node::new(-1, -1);
        let tail = Node::new(-1, -1);
        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(Rc::downgrade(&head));

        LRUCache {
            capacity: capacity as usize,
            map: HashMap::new(),
            head,
            tail,
        }
    }

    fn remove(&self, node: &Rc<RefCell<Node>>) {
        let prev = node.borrow().prev.clone().unwrap().upgrade().unwrap();
        let next = node.borrow().next.clone().unwrap();
        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(Rc::downgrade(&prev));
    }

    fn insert_at_front(&self, node: &Rc<RefCell<Node>>) {
        let first = self.head.borrow().next.clone().unwrap();
        node.borrow_mut().prev = Some(Rc::downgrade(&self.head));
        node.borrow_mut().next = Some(first.clone());
        self.head.borrow_mut().next = Some(node.clone());
        first.borrow_mut().prev = Some(Rc::downgrade(node));
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key).cloned() {
            self.remove(&node);
            self.insert_at_front(&node);
            node.borrow().val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key).cloned() {
            node.borrow_mut().val = value;
            self.remove(&node);
            self.insert_at_front(&node);
        } else {
            if self.map.len() == self.capacity {
                let lru = self.tail.borrow().prev.clone().unwrap().upgrade().unwrap();
                self.remove(&lru);
                self.map.remove(&lru.borrow().key);
            }
            let new_node = Node::new(key, value);
            self.insert_at_front(&new_node);
            self.map.insert(key, new_node);
        }
    }
}