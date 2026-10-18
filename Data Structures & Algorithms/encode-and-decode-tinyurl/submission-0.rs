use std::cell::RefCell;

struct Solution {
    urls: RefCell<Vec<String>>,
}

/** 
 * `&self` means methods take an immutable reference, 
 * but we use `RefCell` for interior mutability so we can push to our vector.
 */
impl Solution {
    fn new() -> Self {
        Solution {
            urls: RefCell::new(Vec::new()),
        }
    }

    // Encodes a URL to a shortened URL.
    fn encode(&self, longUrl: String) -> String {
        let mut urls = self.urls.borrow_mut();
        urls.push(longUrl);
        let id = urls.len() - 1;
        format!("http://tinyurl.com/{}", id)
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, shortUrl: String) -> String {
        // Extract the ID number from the end of the short URL string
        let id_str = shortUrl.split('/').last().unwrap_or("0");
        let id: usize = id_str.parse().unwrap_or(0);
        
        let urls = self.urls.borrow();
        urls[id].clone()
    }
}