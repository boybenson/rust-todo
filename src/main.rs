mod handlers;
mod models;
mod routes;

use routes::create_routes;

#[tokio::main]
async fn main() {
    let app = create_routes();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
