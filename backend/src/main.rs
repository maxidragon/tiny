mod handler;
mod model;
mod route;
mod schema;

use std::sync::Arc;

use axum::http::{header::CONTENT_TYPE, Method};

use dotenv::dotenv;
use tokio::net::TcpListener;

use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

use route::create_router;
use tower_http::cors::{Any, CorsLayer};

pub struct AppState {
    db: MySqlPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must set");
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connected to the database");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the databasse: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState { db: pool.clone() })).layer(cors);
    println!("Server started successfully on port {}", port);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
