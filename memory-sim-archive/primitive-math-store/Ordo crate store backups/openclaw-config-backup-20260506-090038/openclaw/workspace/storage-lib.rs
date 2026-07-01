use anyhow::Result;
use async_trait::async_trait;
use gluesql::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Database {
    glue: Arc<Mutex<Glue<MemoryStorage>>>,
}

impl Database {
    pub async fn new(_path: &str) -> Result<Self> {
        let storage = MemoryStorage::default();
        let glue = Glue::new(storage);
        
        Ok(Self {
            glue: Arc::new(Mutex::new(glue)),
        })
    }

    pub async fn run_migrations(&self) -> Result<()> {
        let mut glue = self.glue.lock().await;
        
        let queries = [
            "CREATE TABLE IF NOT EXISTS posts (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT,
                excerpt TEXT,
                status TEXT NOT NULL,
                tags TEXT,
                created_at TEXT,
                updated_at TEXT
            )",
            "CREATE TABLE IF NOT EXISTS campaigns (
                id TEXT PRIMARY KEY,
                subject TEXT NOT NULL,
                content TEXT,
                status TEXT NOT NULL,
                sent_at TEXT,
                open_count INTEGER DEFAULT 0,
                click_count INTEGER DEFAULT 0
            )",
            "CREATE TABLE IF NOT EXISTS episodes (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                audio_url TEXT,
                duration INTEGER,
                published_at TEXT,
                show_notes TEXT
            )",
            "CREATE TABLE IF NOT EXISTS jobs (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                trigger_event TEXT,
                action TEXT,
                enabled INTEGER DEFAULT 1,
                status TEXT NOT NULL
            )",
            "CREATE TABLE IF NOT EXISTS subscribers (
                id TEXT PRIMARY KEY,
                email TEXT NOT NULL UNIQUE,
                name TEXT,
                subscribed_at TEXT,
                status TEXT NOT NULL
            )",
        ];

        for query in queries {
            glue.execute_stmt(query).await?;
        }

        Ok(())
    }

    pub async fn execute(&self, sql: &str) -> Result<Payload> {
        let mut glue = self.glue.lock().await;
        let mut payloads = glue.execute_stmt(sql).await?;
        Ok(payloads.pop().unwrap_or(Payload::Insert(0)))
    }

    pub async fn query(&self, sql: &str) -> Result<Vec<Row>> {
        let mut glue = self.glue.lock().await;
        let mut rows = glue.execute_stmt(sql).await?;
        // Extract rows from payload
        Ok(rows)
    }
}

#[async_trait]
pub trait Repository<T> {
    async fn get(&self, id: &str) -> Result<Option<T>>;
    async fn save(&self, item: T) -> Result<()>;
    async fn list(&self) -> Result<Vec<T>>;
}

pub type DbContext = Arc<Database>;
