use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use contracts::ModuleInfo;
use runtime::Runtime;
use sqlx::Row;
use std::sync::Arc;

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    modules: Vec<ModuleInfo>,
    status: String,
    post_count: i64,
    campaign_count: i64,
    episode_count: i64,
    job_count: i64,
}

#[derive(Template)]
#[template(path = "module.html")]
struct ModuleTemplate {
    modules: Vec<ModuleInfo>,
    active_module: ModuleInfo,
    module_api_path: String,
}

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate {
    modules: Vec<ModuleInfo>,
}

#[derive(Clone)]
struct UiState {
    runtime: Arc<Runtime>,
}

pub fn create_ui_router(runtime: Arc<Runtime>) -> Router {
    let state = UiState { runtime };

    Router::new()
        .route("/", get(dashboard_handler))
        .route("/ui/blog", get(blog_handler))
        .route("/ui/:module_id", get(module_handler))
        .with_state(state)
}

async fn dashboard_handler(State(state): State<UiState>) -> impl IntoResponse {
    let modules: Vec<ModuleInfo> = state
        .runtime
        .get_modules()
        .iter()
        .map(|m| m.info())
        .collect();

    let post_count = count_rows(&state.runtime, "posts").await;
    let campaign_count = count_rows(&state.runtime, "campaigns").await;
    let episode_count = count_rows(&state.runtime, "episodes").await;
    let job_count = count_rows(&state.runtime, "jobs").await;

    let template = DashboardTemplate {
        modules,
        status: "Running".to_string(),
        post_count,
        campaign_count,
        episode_count,
        job_count,
    };

    template
}

async fn blog_handler(State(state): State<UiState>) -> impl IntoResponse {
    let modules: Vec<ModuleInfo> = state
        .runtime
        .get_modules()
        .iter()
        .map(|m| m.info())
        .collect();

    BlogTemplate { modules }
}

async fn module_handler(
    State(state): State<UiState>,
    Path(module_id): Path<String>,
) -> impl IntoResponse {
    let modules: Vec<ModuleInfo> = state
        .runtime
        .get_modules()
        .iter()
        .map(|m| m.info())
        .collect();

    let active_module = modules
        .iter()
        .find(|m| m.id == module_id)
        .cloned()
        .unwrap_or_else(|| ModuleInfo {
            id: "unknown".to_string(),
            name: "Unknown Module".to_string(),
            version: "0.0.0".to_string(),
            capabilities: vec![],
        });

    let template = ModuleTemplate {
        modules,
        active_module,
        module_api_path: module_api_path(&module_id),
    };

    template
}

async fn count_rows(runtime: &Runtime, table: &str) -> i64 {
    let query = format!("SELECT COUNT(1) AS count FROM {}", table);
    match sqlx::query(&query).fetch_one(&runtime.db.pool).await {
        Ok(row) => row.get::<i64, _>("count"),
        Err(_) => 0,
    }
}

fn module_api_path(module_id: &str) -> String {
    match module_id {
        "blog" => "/api/posts".to_string(),
        "newsletter" => "/api/campaigns".to_string(),
        "podcast" => "/api/episodes".to_string(),
        "automation" => "/api/jobs".to_string(),
        _ => format!("/api/{}", module_id),
    }
}
