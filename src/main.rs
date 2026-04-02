mod config;
mod error;
mod handlers;
mod middleware;
mod model;
mod routes;
mod state;
use config::database::conn_db;
use state::AppState;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::middleware::middleware_satu;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tasks=info,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = conn_db().await;

    let app_state = AppState { db_pool: pool };

    let app_layer = ServiceBuilder::new()
        .layer(axum::middleware::from_fn(middleware_satu))
        .layer(TraceLayer::new_for_http());

    let app = axum::Router::new()
        .nest("/api", routes::router_task::task_router())
        .layer(app_layer)
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    tracing::info!(
        "Listening on http://{}",
        listener.local_addr().unwrap().to_string()
    );

    axum::serve(listener, app).await.unwrap();
}
