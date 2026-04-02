use axum::{extract::Request, middleware::Next, response::Response};

pub async fn middleware_satu(req: Request, next: Next) -> Response {
    println!("Middleware KU");
    let response = next.run(req).await;
    response
}
