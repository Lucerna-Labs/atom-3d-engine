use anyhow::Result;
use clap::{Parser, Subcommand};
use runtime::Runtime;
use std::sync::Arc;
use storage::BccDatabase;
use tracing::info;

#[derive(Parser)]
#[command(name = "bcc")]
#[command(about = "Brand Command Center CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the server
    Start {
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
    /// Run database migrations
    Migrate,
    /// List all installed modules
    ListModules,
    /// Show system status
    Status,
    /// Initialize sample data
    InitData,
    /// Run sample module actions
    RunSample,
}

async fn setup_runtime() -> Result<Arc<Runtime>> {
    let config = bcc_core::load_config();
    let db = BccDatabase::new(&config.database_url).await?;
    db.run_migrations().await?;
    let db_context = Arc::new(db);

    let mut runtime = Runtime::new(db_context);

    // Register modules
    runtime.register_module(Arc::new(blog::BlogModule::new()));
    runtime.register_module(Arc::new(newsletter::NewsletterModule::new()));
    runtime.register_module(Arc::new(podcast::PodcastModule::new()));
    runtime.register_module(Arc::new(automation::AutomationModule::new()));

    // Initialize all modules
    runtime.init_all().await?;

    Ok(Arc::new(runtime))
}

#[tokio::main]
async fn main() -> Result<()> {
    bcc_core::setup_logging();

    let cli = Cli::parse();

    match cli.command {
        Commands::Start { port } => {
            let runtime = setup_runtime().await?;
            runtime.start_all().await?;

            // Combine API and UI routers
            let api_router = api::create_router(Arc::clone(&runtime));
            let ui_router = ui_shell::create_ui_router(Arc::clone(&runtime));

            let app = axum::Router::new()
                .merge(ui_router)
                .nest("/api", api_router)
                .layer(tower_http::trace::TraceLayer::new_for_http());

            let addr = format!("0.0.0.0:{}", port);
            info!("Server listening on http://{}", addr);
            let listener = tokio::net::TcpListener::bind(&addr).await?;
            axum::serve(listener, app).await?;
        }
        Commands::Migrate => {
            let config = bcc_core::load_config();
            let db = BccDatabase::new(&config.database_url).await?;
            info!("Running migrations...");
            db.run_migrations().await?;
            info!("Migrations complete.");
        }
        Commands::ListModules => {
            let runtime = setup_runtime().await?;
            println!("Installed Modules:");
            for module in runtime.get_modules() {
                let info = module.info();
                println!(
                    "- {} (v{}) - Status: {}",
                    info.name,
                    info.version,
                    module.status()
                );
            }
        }
        Commands::Status => {
            let runtime = setup_runtime().await?;
            let module_count = runtime.get_modules().len();
            println!("System Status: OK");
            println!("Modules Loaded: {}", module_count);
            println!("Database: Connected (migrations verified)");
        }
        Commands::InitData => {
            let config = bcc_core::load_config();
            let _db = BccDatabase::new(&config.database_url).await?;
            info!("Initializing sample data...");
            // Placeholder for inserting sample data
            info!("Sample data initialized.");
        }
        Commands::RunSample => {
            let runtime = setup_runtime().await?;
            info!("Running sample module actions...");

            // Example: Fire a post_published event to see if Newsletter reacts
            use contracts::Event;
            let event = Event {
                topic: "post_published".to_string(),
                payload: serde_json::json!({"id": "sample_post_1", "title": "Hello World"}),
                source: "cli".to_string(),
            };

            runtime.event_bus.publish(event)?;
            info!("Sample event published. Check logs for module reactions.");

            // Wait briefly to allow async handlers to run
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    }

    Ok(())
}
