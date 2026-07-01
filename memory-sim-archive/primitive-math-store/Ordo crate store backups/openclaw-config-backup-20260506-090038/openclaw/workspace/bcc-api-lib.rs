use axum::{extract::State, http::StatusCode, routing::{get, post, put, delete}, Json, Router};
use bcc_core::HealthStatus;
use contracts::ModuleInfo;
use runtime::Runtime;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use storage::{Database, Post as DbPost};
use tower_http::trace::TraceLayer;

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
        .route("/posts/:id", get(get_post).put(update_post).delete(delete_post))
        .route("/campaigns", get(list_campaigns).post(create_campaign))
        .route("/episodes", get(list_episodes).post(create_episode))
        .route("/jobs", get(list_jobs).post(create_job))
        .with_state(state)
}

async fn health_check(State(state): State<AppState>) -> Json<HealthStatus> {
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

#[derive(Debug, Serialize, Deserialize)]
struct CreatePostRequest {
    title: String,
    content: String,
    excerpt: Option<String>,
    status: Option<String>,
    tags: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdatePostRequest {
    title: Option<String>,
    content: Option<String>,
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

async fn list_posts(State(state): State<AppState>) -> Result<Json<Vec<DbPost>>, StatusCode> {
    let posts = state.runtime.db.list_posts()
        .await
        .map_err(|e| {
            tracing::error!("Error fetching posts: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(posts))
}

async fn get_post(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<DbPost>, StatusCode> {
    let post = state.runtime.db.get_post(&id)
        .await
        .map_err(|e| {
            tracing::error!("Error fetching post: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(post))
}

async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<DbPost>), StatusCode> {
    let id = make_id("post");
    let now = chrono::Utc::now().to_rfc3339();
    
    let post = DbPost {
        id: id.clone(),
        title: payload.title,
        content: Some(payload.content),
        excerpt: payload.excerpt,
        status: payload.status.unwrap_or_else(|| "draft".to_string()),
        tags: payload.tags,
        created_at: Some(now.clone()),
        updated_at: Some(now),
    };

    state.runtime.db.create_post(&post)
        .await
        .map_err(|e| {
            tracing::error!("Error creating post: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok((StatusCode::CREATED, Json(post)))
}

async fn update_post(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<DbPost>, StatusCode> {
    let mut post = state.runtime.db.get_post(&id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if let Some(title) = payload.title {
        post.title = title;
    }
    if let Some(content) = payload.content {
        post.content = Some(content);
    }
    if let Some(excerpt) = payload.excerpt {
        post.excerpt = Some(excerpt);
    }
    if let Some(status) = payload.status {
        post.status = status;
    }
    if let Some(tags) = payload.tags {
        post.tags = Some(tags);
    }
    post.updated_at = Some(chrono::Utc::now().to_rfc3339());

    state.runtime.db.update_post(&post)
        .await
        .map_err(|e| {
            tracing::error!("Error updating post: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(post))
}

async fn delete_post(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<StatusCode, StatusCode> {
    state.runtime.db.delete_post(&id)
        .await
        .map_err(|e| {
            tracing::error!("Error deleting post: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(StatusCode::NO_CONTENT)
}

async fn list_campaigns(State(_state): State<AppState>) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    Ok(Json(vec![]))
}

async fn create_campaign(
    State(_state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    Ok((StatusCode::CREATED, Json(payload)))
}

async fn list_episodes(State(_state): State<AppState>) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    Ok(Json(vec![]))
}

async fn create_episode(
    State(_state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    Ok((StatusCode::CREATED, Json(payload)))
}

async fn list_jobs(State(_state): State<AppState>) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    Ok(Json(vec![]))
}

async fn create_job(
    State(_state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    Ok((StatusCode::CREATED, Json(payload)))
}
