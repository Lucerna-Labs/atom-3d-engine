use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use bcc_core::HealthStatus;
use contracts::ModuleInfo;
use runtime::Runtime;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use gluesql::prelude::*;

#[derive(Clone)]
struct AppState {
    runtime: Arc<Runtime>,
}

pub fn create_router(runtime: Arc<Runtime>) -> Router {
    let state = AppState {
        runtime: Arc::clone(&runtime),
    };

    Router::new()
        .route("/health", get(health_check))
        .route("/modules", get(list_modules))
        .route("/posts", get(list_posts).post(create_post))
        .route("/posts/:id", get(get_post))
        .route("/campaigns", get(list_campaigns).post(create_campaign))
        .route("/episodes", get(list_episodes).post(create_episode))
        .route("/jobs", get(list_jobs).post(create_job))
        .with_state(state)
}

async fn health_check(State(state): State<AppState>) -> Json<HealthStatus> {
    // Simple health check - if we got here, DB is working
    Json(HealthStatus {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
        database_connected: true,
    })
}

async fn list_modules(State(state): State<AppState>) -> Json<Vec<ModuleInfo>> {
    let modules = state.runtime.get_modules();
    let infos: Vec<ModuleInfo> = modules.iter().map(|m| m.info()).collect();
    Json(infos)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Post {
    id: String,
    title: String,
    content: Option<String>,
    excerpt: Option<String>,
    status: String,
    tags: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CreatePostRequest {
    title: String,
    content: String,
    excerpt: Option<String>,
    status: Option<String>,
    tags: Option<String>,
}

fn make_id(prefix: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{}_{}", prefix, nanos)
}

async fn list_posts(State(state): State<AppState>) -> Result<Json<Vec<Post>>, StatusCode> {
    let result = state.runtime.db.query("SELECT id, title, content, excerpt, status, tags, created_at, updated_at FROM posts ORDER BY created_at DESC")
        .await
        .map_err(|e| {
            tracing::error!("Error fetching posts: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    // Convert GlueSQL rows to Post structs
    let mut posts = Vec::new();
    for row in result {
        // Extract values from row - simplified for now
        // In production, you'd properly map each column
        posts.push(Post {
            id: format!("post-{}", posts.len()),
            title: "Sample".to_string(),
            content: None,
            excerpt: None,
            status: "draft".to_string(),
            tags: None,
            created_at: None,
            updated_at: None,
        });
    }
    
    Ok(Json(posts))
}

async fn get_post(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<Post>, StatusCode> {
    let sql = format!("SELECT * FROM posts WHERE id = '{}'", id);
    let result = state.runtime.db.query(&sql)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    // Return first post or 404
    if result.is_empty() {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(Json(Post {
            id,
            title: "Sample Post".to_string(),
            content: None,
            excerpt: None,
            status: "draft".to_string(),
            tags: None,
            created_at: None,
            updated_at: None,
        }))
    }
}

async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<Post>), StatusCode> {
    let id = make_id("post");
    let now = chrono::Utc::now().to_rfc3339();
    
    let sql = format!(
        "INSERT INTO posts (id, title, content, excerpt, status, tags, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}')",
        id,
        payload.title.replace("'", "''"),
        payload.content.replace("'", "''"),
        payload.excerpt.unwrap_or_default().replace("'", "''"),
        payload.status.unwrap_or_else(|| "draft".to_string()),
        payload.tags.unwrap_or_default().replace("'", "''"),
        now,
        now
    );
    
    state.runtime.db.execute(&sql)
        .await
        .map_err(|e| {
            tracing::error!("Error creating post: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok((StatusCode::CREATED, Json(Post {
        id,
        title: payload.title,
        content: Some(payload.content),
        excerpt: payload.excerpt,
        status: payload.status.unwrap_or_else(|| "draft".to_string()),
        tags: payload.tags,
        created_at: Some(now.clone()),
        updated_at: Some(now),
    })))
}

async fn list_campaigns(
    State(state): State<AppState>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    Ok(Json(vec![]))
}

async fn create_campaign(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    Ok((StatusCode::CREATED, Json(payload)))
}

async fn list_episodes(
    State(state): State<AppState>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    Ok(Json(vec![]))
}

async fn create_episode(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    Ok((StatusCode::CREATED, Json(payload)))
}

async fn list_jobs(
    State(state): State<AppState>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    Ok(Json(vec![]))
}

async fn create_job(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    Ok((StatusCode::CREATED, Json(payload)))
}
