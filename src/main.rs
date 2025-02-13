use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

mod handler;
mod image_generator;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/:size", get(handler::generate_image_with_size))
        .route("/:size/:bg/:fg/text=:text", get(handler::generate_image))
        .route("/:size/:bg/:fg", get(handler::generate_image_without_text));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
} 