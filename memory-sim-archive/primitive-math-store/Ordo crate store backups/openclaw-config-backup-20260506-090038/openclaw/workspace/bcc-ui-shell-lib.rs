use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use contracts::ModuleInfo;
use runtime::Runtime;
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

    let post_count = state.runtime.db.list_posts().await.map(|p| p.len() as i64).unwrap_or(0);
    let campaign_count = 0i64;
    let episode_count = 0i64;
    let job_count = 0i64;

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

fn module_api_path(module_id: &str) -> String {
    match module_id {
        "blog" => "/api/posts".to_string(),
        "newsletter" => "/api/campaigns".to_string(),
        "podcast" => "/api/episodes".to_string(),
        "automation" => "/api/jobs".to_string(),
        _ => format!("/api/{}", module_id),
    }
}
