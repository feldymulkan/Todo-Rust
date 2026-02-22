mod config;
mod state;
mod handlers;
mod routes;
mod model;
mod error;
use state::AppState;
use tokio::net::TcpListener;
use config::database::conn_db;


#[tokio::main]
async fn main() {
    let pool = conn_db().await;

    let app_state = AppState{
        db_pool: pool
    };

    let app = axum::Router::new()
        .nest("/api", routes::router_task::task_router())
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    println!("Listening on http://{}", listener.local_addr().unwrap().to_string());

    axum::serve(listener, app)
        .await
        .unwrap();

}
