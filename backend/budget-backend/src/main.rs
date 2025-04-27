use axum::{
    http::{HeaderValue, Method},
    routing::{get,},
    Router,
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use sea_orm::*;

mod purchases;
mod categories;
mod entities;
mod util;
mod byid;

use entities::{prelude::*, *};

const DATABASE_URL: &str = "sqlite://budget.db";

#[tokio::main]
async fn main() {
    // Init logging
    tracing_subscriber::fmt::init();

    // Setup database
    let db = Database::connect(DATABASE_URL).await.unwrap();

    let count = Category::find().count(&db).await;
    if count == Ok(0) {
        let other = category::ActiveModel {
            name: ActiveValue::Set("Other".to_owned()),
            budget: ActiveValue::Set(None),
            ..Default::default()
        };
        let _res = Category::insert(other).exec(&db).await.expect("Failed to insert category");
    }

    // Build app
    let app = Router::new()
        .route("/categories", get(categories::get).post(categories::post))
        .route("/purchases", get(purchases::get).post(purchases::post))
        .route(
            "/categories/{cat_id}",
            get(byid::cat_get),
        )
        .route(
            "/purchases/{pur_id}",
            get(byid::pur_get),
        )
        .with_state(db.clone())
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods(vec![Method::GET, Method::POST])
                .allow_headers(vec![axum::http::header::CONTENT_TYPE]),
        )
        .layer(TraceLayer::new_for_http());

    // Bind to address/port and run
    let listener = tokio::net::TcpListener::bind("localhost:8000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}



/*
DATABASE_URL="sqlite://budget.db" sea-orm-cli migrate refresh
*/


//remakes entities
/*
sea-orm-cli generate entity \
    -u sqlite://budget.db \
    -o src/entities
*/