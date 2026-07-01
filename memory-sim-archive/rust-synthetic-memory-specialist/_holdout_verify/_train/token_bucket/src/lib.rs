#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grant {
    pub node: String,
    pub allowed: bool,
    pub remaining: i32,
    pub signals: Vec<String>,
}

struct Bucket {
    node: String,
    tokens: i32,
}

pub struct TokenBucket {
    buckets: Vec<Bucket>,
}

impl TokenBucket {
    pub fn new() -> Self {
        TokenBucket { buckets: Vec::new() }
    }

    pub fn set_capacity(&mut self, node: &str, capacity: i32) {
        match self.buckets.iter_mut().find(|b| b.node == node) {
            Some(bucket) => bucket.tokens = capacity,
            None => self.buckets.push(Bucket {
                node: node.to_string(),
                tokens: capacity,
            }),
        }
    }

    pub fn request(&mut self, node: &str) -> Grant {
        match self.buckets.iter_mut().find(|b| b.node == node) {
            Some(bucket) if bucket.tokens > 0 => {
                bucket.tokens -= 1;
                Grant {
                    node: node.to_string(),
                    allowed: true,
                    remaining: bucket.tokens,
                    signals: Vec::new(),
                }
            }
            Some(bucket) => Grant {
                node: node.to_string(),
                allowed: false,
                remaining: bucket.tokens,
                signals: vec![format!("deny:{}", node)],
            },
            None => Grant {
                node: node.to_string(),
                allowed: false,
                remaining: 0,
                signals: vec![format!("deny:{}", node)],
            },
        }
    }
}
