use std::collections::{HashMap, HashSet};

struct Twitter {
    time: i32,
    tweets: HashMap<i32, Vec<(i32, i32)>>,
    following: HashMap<i32, HashSet<i32>>
}

impl Twitter {
    pub fn new() -> Self {
        Twitter {
            time: 0,
            tweets: HashMap::new(),
            following: HashMap::new()
        }
    }

    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.time += 1;
        self.tweets.entry(user_id).or_insert(Vec::new()).push((self.time, tweet_id));
    }
fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut heap: BinaryHeap<(i32, i32, i32, usize)> = BinaryHeap::new();
        // (timestamp, tweetId, authorId, index_into_that_author's_tweet_list)

        let mut relevant_users: HashSet<i32> = HashSet::new();
        relevant_users.insert(user_id); // always see your own tweets
        if let Some(followees) = self.following.get(&user_id) {
            for &f in followees {
                relevant_users.insert(f);
            }
        }

        for &uid in &relevant_users {
            if let Some(tweet_list) = self.tweets.get(&uid) {
                if !tweet_list.is_empty() {
                    let last_idx = tweet_list.len() - 1;
                    let (ts, tid) = tweet_list[last_idx];
                    heap.push((ts, tid, uid, last_idx));
                }
            }
        }

        let mut result = Vec::new();
        while result.len() < 10 {
            match heap.pop() {
                Some((ts, tid, uid, idx)) => {
                    result.push(tid);
                    if idx > 0 {
                        let tweet_list = &self.tweets[&uid];
                        let (next_ts, next_tid) = tweet_list[idx - 1];
                        heap.push((next_ts, next_tid, uid, idx - 1));
                    }
                }
                None => break,
            }
        }

        result
    }

    pub fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.following.entry(follower_id).or_insert_with(HashSet::new).insert(followee_id);
    }

    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(set) = self.following.get_mut(&follower_id) {
            set.remove(&followee_id);
        }
    }
}
