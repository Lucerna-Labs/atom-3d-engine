use anyhow::Result;
use async_trait::async_trait;
use redb::{Database, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

// Table definitions
const POSTS: TableDefinition<&[u8], &[u8]> = TableDefinition::new("posts");
const CAMPAIGNS: TableDefinition<&[u8], &[u8]> = TableDefinition::new("campaigns");
const EPISODES: TableDefinition<&[u8], &[u8]> = TableDefinition::new("episodes");
const JOBS: TableDefinition<&[u8], &[u8]> = TableDefinition::new("jobs");
const SUBSCRIBERS: TableDefinition<&[u8], &[u8]> = TableDefinition::new("subscribers");

#[derive(Clone)]
pub struct BccDatabase {
    db: Arc<Database>,
}

impl BccDatabase {
    pub async fn new(path: &str) -> Result<Self> {
        let db = Database::create(path)?;
        Ok(Self { db: Arc::new(db) })
    }

    pub async fn run_migrations(&self) -> Result<()> {
        // Create tables
        let write_txn = self.db.begin_write()?;
        {
            let _ = write_txn.open_table(POSTS)?;
            let _ = write_txn.open_table(CAMPAIGNS)?;
            let _ = write_txn.open_table(EPISODES)?;
            let _ = write_txn.open_table(JOBS)?;
            let _ = write_txn.open_table(SUBSCRIBERS)?;
        }
        write_txn.commit()?;
        Ok(())
    }

    // Key format: timestamp_nanoseconds|id for chronological ordering
    fn make_key(prefix: u64, id: &str) -> Vec<u8> {
        format!("{:020}|{}", prefix, id).into_bytes()
    }

    fn parse_key(key: &[u8]) -> Option<(&str, &str)> {
        let s = std::str::from_utf8(key).ok()?;
        let parts: Vec<&str> = s.split('|').collect();
        if parts.len() == 2 {
            Some((parts[0], parts[1]))
        } else {
            None
        }
    }

    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }

    // Posts
    pub async fn create_post(&self, post: &Post) -> Result<()> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(POSTS)?;
            let key = Self::make_key(Self::now(), &post.id);
            let value = serde_json::to_vec(post)?;
            table.insert(key.as_slice(), value.as_slice())?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub async fn list_posts(&self) -> Result<Vec<Post>> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(POSTS)?;
        let mut posts = Vec::new();
        
        for entry in table.range::<&[u8]>(..)? {
            let (_, value) = entry?;
            let post: Post = serde_json::from_slice(value.value())?;
            posts.push(post);
        }
        
        // Reverse to get newest first (redb stores in ascending order)
        posts.reverse();
        Ok(posts)
    }

    pub async fn get_post(&self, id: &str) -> Result<Option<Post>> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(POSTS)?;
        
        for entry in table.range::<&[u8]>(..)? {
            let (key, value) = entry?;
            if let Some((_, stored_id)) = Self::parse_key(key.value()) {
                if stored_id == id {
                    let post: Post = serde_json::from_slice(value.value())?;
                    return Ok(Some(post));
                }
            }
        }
        Ok(None)
    }

    pub async fn delete_post(&self, id: &str) -> Result<()> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(POSTS)?;
            
            // Find and delete the key
            for entry in table.range::<&[u8]>(..)? {
                let (key, _) = entry?;
                if let Some((_, stored_id)) = Self::parse_key(key.value()) {
                    if stored_id == id {
                        table.remove(key.value())?;
                        break;
                    }
                }
            }
        }
        write_txn.commit()?;
        Ok(())
    }

    pub async fn update_post(&self, post: &Post) -> Result<()> {
        // Delete old, insert new (simpler than finding exact key)
        self.delete_post(&post.id).await?;
        self.create_post(post).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: Option<String>,
    pub excerpt: Option<String>,
    pub status: String,
    pub tags: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[async_trait]
pub trait Repository<T> {
    async fn get(&self, id: &str) -> Result<Option<T>>;
    async fn save(&self, item: T) -> Result<()>;
    async fn list(&self) -> Result<Vec<T>>;
}

pub type DbContext = Arc<BccDatabase>;
