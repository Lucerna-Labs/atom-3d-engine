use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use bcc_core::HealthStatus;
use contracts::ModuleInfo;
use runtime::Runtime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
struct AppState {
    runtime: Arc<Runtime>,
}

pub fn create_router(runtime: Arc<Runtime>) -> Router {
    let state = AppState {
        runtime: Arc::clone(&runtime),
    };

    let mut router = Router::new()
        .route("/health", get(health_check))
        .route("/modules", get(list_modules))
        .route("/posts", get(list_posts).post(create_post))
        .route("/posts/:id", get(get_post).put(update_post).delete(delete_post))
        .route("/campaigns", get(list_campaigns).post(create_campaign))
        .route("/episodes", get(list_episodes).post(create_episode))
        .route("/jobs", get(list_jobs).post(create_job))
        .with_state(state);

    for module in runtime.get_modules() {
        if let Some(module_router) = module.routes() {
            let path = format!("/{}", module.info().id);
            router = router.nest(&path, module_router);
        }
    }

    router.layer(TraceLayer::new_for_http())
}

async fn health_check(State(state): State<AppState>) -> Json<HealthStatus> {
    let db_ok = sqlx::query("SELECT 1")
        .execute(&state.runtime.db.pool)
        .await
        .is_ok();

    Json(HealthStatus {
        status: if db_ok { "ok" } else { "degraded" }.to_string(),
        version: "0.1.0".to_string(),
        database_connected: db_ok,
    })
}

async fn list_modules(State(state): State<AppState>) -> Json<Vec<ModuleInfo>> {
    let modules = state.runtime.get_modules();
    let infos: Vec<ModuleInfo> = modules.iter().map(|m| m.info()).collect();
    Json(infos)
}

#[derive(Debug, Clone, Serialize, FromRow)]
struct PostRecord {
    id: String,
    title: String,
    content: Option<String>,
    excerpt: Option<String>,
    status: String,
    tags: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct CreatePostRequest {
    title: String,
    content: String,
    excerpt: Option<String>,
    status: Option<String>,
    tags: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct UpdatePostRequest {
    title: Option<String>,
    content: Option<String>,
    excerpt: Option<String>,
    status: Option<String>,
    tags: Option<String>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
struct CampaignRecord {
    id: String,
    subject: String,
    status: String,
}

#[derive(Debug, Clone, Deserialize)]
struct CreateCampaignRequest {
    subject: String,
}

#[derive(Debug, Clone, Serialize, FromRow)]
struct EpisodeRecord {
    id: String,
    title: String,
    audio_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct CreateEpisodeRequest {
    title: String,
    audio_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
struct JobRecord {
    id: String,
    name: String,
    status: String,
}

#[derive(Debug, Clone, Deserialize)]
struct CreateJobRequest {
    name: String,
}

fn make_id(prefix: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{}_{}", prefix, nanos)
}

async fn list_posts(State(state): State<AppState>) -> Result<Json<Vec<PostRecord>>, StatusCode> {
    let rows = sqlx::query_as::<_, PostRecord>(
        "SELECT id, title, content, excerpt, status, tags, created_at, updated_at FROM posts ORDER BY created_at DESC",
    )
    .fetch_all(&state.runtime.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Error fetching posts: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(rows))
}

async fn get_post(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<PostRecord>, StatusCode> {
    let row = sqlx::query_as::<_, PostRecord>(
        "SELECT id, title, content, excerpt, status, tags, created_at, updated_at FROM posts WHERE id = ?",
    )
    .bind(&id)
    .fetch_one(&state.runtime.db.pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(row))
}

async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<PostRecord>), StatusCode> {
    let id = make_id("post");
    let now = chrono::Utc::now().to_rfc3339();
    
    let record = PostRecord {
        id: id.clone(),
        title: payload.title,
        content: Some(payload.content),
        excerpt: payload.excerpt,
        status: payload.status.unwrap_or_else(|| "draft".to_string()),
        tags: payload.tags,
        created_at: Some(now.clone()),
        updated_at: Some(now),
    };

    sqlx::query("INSERT INTO posts (id, title, content, excerpt, status, tags, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(&record.id)
        .bind(&record.title)
        .bind(&record.content)
        .bind(&record.excerpt)
        .bind(&record.status)
        .bind(&record.tags)
        .bind(&record.created_at)
        .bind(&record.updated_at)
        .execute(&state.runtime.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error creating post: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok((StatusCode::CREATED, Json(record)))
}

async fn update_post(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<PostRecord>, StatusCode> {
    let now = chrono::Utc::now().to_rfc3339();
    
    let mut query_parts = Vec::new();
    let mut binds: Vec<&str> = Vec::new();
    
    if let Some(title) = &payload.title {
        query_parts.push("title = ?");
        binds.push(title);
    }
    if let Some(content) = &payload.content {
        query_parts.push("content = ?");
        binds.push(content);
    }
    if let Some(excerpt) = &payload.excerpt {
        query_parts.push("excerpt = ?");
        binds.push(excerpt);
    }
    if let Some(status) = &payload.status {
        query_parts.push("status = ?");
        binds.push(status);
    }
    if let Some(tags) = &payload.tags {
        query_parts.push("tags = ?");
        binds.push(tags);
    }
    
    if query_parts.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    query_parts.push("updated_at = ?");
    binds.push(&now);
    binds.push(&id);
    
    let query = format!(
        "UPDATE posts SET {} WHERE id = ?",
        query_parts.join(", ")
    );
    
    let mut sqlx_query = sqlx::query(&query);
    for bind in binds.iter().take(binds.len() - 1) {
        sqlx_query = sqlx_query.bind(bind);
    }
    sqlx_query = sqlx_query.bind(&id);
    
    sqlx_query.execute(&state.runtime.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error updating post: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    get_post_inner(&state, &id).await
}

async fn get_post_inner(state: &AppState, id: &str) -> Result<Json<PostRecord>, StatusCode> {
    let row = sqlx::query_as::<_, PostRecord>(
        "SELECT id, title, content, excerpt, status, tags, created_at, updated_at FROM posts WHERE id = ?",
    )
    .bind(id)
    .fetch_one(&state.runtime.db.pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(row))
}

async fn delete_post(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query("DELETE FROM posts WHERE id = ?")
        .bind(&id)
        .execute(&state.runtime.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error deleting post: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(StatusCode::NO_CONTENT)
}

async fn list_campaigns(
    State(state): State<AppState>,
) -> Result<Json<Vec<CampaignRecord>>, StatusCode> {
    let rows = sqlx::query_as::<_, CampaignRecord>(
        "SELECT id, subject, status FROM campaigns ORDER BY rowid DESC",
    )
    .fetch_all(&state.runtime.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(rows))
}

async fn create_campaign(
    State(state): State<AppState>,
    Json(payload): Json<CreateCampaignRequest>,
) -> Result<(StatusCode, Json<CampaignRecord>), StatusCode> {
    let record = CampaignRecord {
        id: make_id("campaign"),
        subject: payload.subject,
        status: "draft".to_string(),
    };

    sqlx::query("INSERT INTO campaigns (id, subject, status) VALUES (?, ?, ?)")
        .bind(&record.id)
        .bind(&record.subject)
        .bind(&record.status)
        .execute(&state.runtime.db.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(record)))
}

async fn list_episodes(
    State(state): State<AppState>,
) -> Result<Json<Vec<EpisodeRecord>>, StatusCode> {
    let rows = sqlx::query_as::<_, EpisodeRecord>(
        "SELECT id, title, audio_url FROM episodes ORDER BY rowid DESC",
    )
    .fetch_all(&state.runtime.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(rows))
}

async fn create_episode(
    State(state): State<AppState>,
    Json(payload): Json<CreateEpisodeRequest>,
) -> Result<(StatusCode, Json<EpisodeRecord>), StatusCode> {
    let record = EpisodeRecord {
        id: make_id("episode"),
        title: payload.title,
        audio_url: payload.audio_url,
    };

    sqlx::query("INSERT INTO episodes (id, title, audio_url) VALUES (?, ?, ?)")
        .bind(&record.id)
        .bind(&record.title)
        .bind(&record.audio_url)
        .execute(&state.runtime.db.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(record)))
}

async fn list_jobs(State(state): State<AppState>) -> Result<Json<Vec<JobRecord>>, StatusCode> {
    let rows =
        sqlx::query_as::<_, JobRecord>("SELECT id, name, status FROM jobs ORDER BY rowid DESC")
            .fetch_all(&state.runtime.db.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(rows))
}

async fn create_job(
    State(state): State<AppState>,
    Json(payload): Json<CreateJobRequest>,
) -> Result<(StatusCode, Json<JobRecord>), StatusCode> {
    let record = JobRecord {
        id: make_id("job"),
        name: payload.name,
        status: "queued".to_string(),
    };

    sqlx::query("INSERT INTO jobs (id, name, status) VALUES (?, ?, ?)")
        .bind(&record.id)
        .bind(&record.name)
        .bind(&record.status)
        .execute(&state.runtime.db.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(record)))
}
