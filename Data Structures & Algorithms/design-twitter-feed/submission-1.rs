use std::collections::{HashMap, HashSet};

struct Twitter{
    time: i32,
    tweets: HashMap<i32, Vec<(i32, i32)>>, //userId, timestamp of the post and the post Id
    followings: HashMap<i32,HashSet<i32>> //userId, the people that userId follow
}

impl Twitter{
    pub fn new()->Self{
        Twitter{
            time: 0,
            tweets: HashMap::new(),
            followings: HashMap::new()
        }
    }
    
    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32){
        self.time += 1; 
        self.tweets.entry(user_id).or_insert(Vec::new()).push((self.time, tweet_id));
    }
    
    pub fn get_news_feed(&self, user_id: i32)->Vec<i32>{
        // 
        
        let mut heap: BinaryHeap<(i32, i32, usize, i32)> = BinaryHeap::new(); //time, tweet_id, index, author_id
        
        let mut relavant_user: HashSet<i32> = HashSet::new();
        
        relavant_user.insert(user_id);
        
       if let Some(followees) = self.followings.get(&user_id) {
           for &f in followees {
                relavant_user.insert(f);
           }
       }
       
       for &uid in &relavant_user {
           if let Some(tweets) = self.tweets.get(&uid) {
               if !tweets.is_empty(){
                   let last_idx = tweets.len() - 1;
                   let (ts, tid) = tweets[last_idx];
                   heap.push((ts, tid, last_idx, uid));
               }
           }
       }
       
       let mut result = Vec::new();
       
       while result.len() < 10 {
           match heap.pop(){
               Some((time, tweet_id, index, author_id))=>{
                   result.push(tweet_id);
                   if index > 0 { 
                       let tweet_list = &self.tweets[&author_id];
                       let (next_ts, next_tid) = tweet_list[index-1];
                       heap.push((next_ts, next_tid, index-1, author_id)); //time, tweet_id, index, author_id
                   }
               },
               None=>break
           }
       }
       
       
       
       result
       
        
    } 
    
    pub fn follow(&mut self, followees_id: i32, follower_id: i32){
        self.followings.entry(followees_id).or_insert(HashSet::new()).insert(follower_id);
    }
    
     pub fn unfollow(&mut self, followees_id: i32, follower_id: i32){
       if let Some(mut followings) = self.followings.get_mut(&followees_id) {
           followings.remove(&follower_id);
       }
    }
    
    
}
