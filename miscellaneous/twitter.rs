use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};

const FEED_LENGTH: usize = 10;

struct Twitter {
    // List of users followed by each user ID
    following_map: HashMap<i32, HashSet<i32>>,
    // For each user, list of (timestamp, tweet_id)
    tweet_map: HashMap<i32, VecDeque<(u64, i32)>>,
    // News feed for each user
    news_feed: HashMap<i32, VecDeque<i32>>,
    timestamp: u64
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {

    fn new() -> Self {
        // Each user is a follower of themselves
        let mut following_map = HashMap::with_capacity(500);
        for user in 1..=500 {
            following_map.insert(user, HashSet::from([user]));
        }

        Self {
            following_map: following_map,
            tweet_map: HashMap::new(),
            news_feed: HashMap::new(),
            timestamp: 0
        }
    }
    
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        let tweet = (self.timestamp, tweet_id); 
        self.tweet_map
            .entry(user_id)
            .and_modify(|v| v.push_front(tweet))
            .or_insert(VecDeque::from([tweet]));

        self.timestamp += 1;
    }
    
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut feeds_minheap = BinaryHeap::with_capacity(FEED_LENGTH);

        let mut iters = self.following_map.get(&user_id)
                            .expect("user not found in following_map")
                            .iter()
                            .filter_map(|u| { self.tweet_map.get(u)}).
                            .map(|v| v.iter());

        'outer: loop {
            for iter in iters {
                if let Some(&tweet) = iter.next() {
                    if feeds_minheap.len() < FEED_LENGTH {
                        feeds_minheap.push(std::cmp::Reverse(tweet));
                    }
                    else {
                        let oldest_tweet = feeds_minheap.peek().unwrap().0;
                        if (*tweet).0 > (*oldest_tweet).0 {
                            feeds_minheap.pop();
                            feeds_minheap.push(std::cmp::Reverse(tweet))
                        }
                        else {
                            break 'outer;
                        }
                    }
                }
            }
        }

        let mut result = Vec::with_capacity(FEED_LENGTH);
        while !feeds_minheap.is_empty() {
            result.push(feeds_minheap.pop().unwrap().0.1);
        }

        result.into_iter().rev().collect()
    }
    
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.following_map.entry(follower_id)
            .and_modify(|h| { h.insert(followee_id) ;})
            .or_insert(HashSet::from([followee_id]));
    }
    
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.following_map
            .get_mut(&follower_id)
            .expect("user not found in following_map")
            .remove(&followee_id);
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */