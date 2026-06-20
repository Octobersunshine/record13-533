mod models;
mod state;
mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use state::AppState;

#[tokio::main]
async fn main() {
    let app_state = AppState::new();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/scripts", get(handlers::list_scripts).post(handlers::create_script))
        .route("/scripts/:id", get(handlers::get_script).put(handlers::update_script).delete(handlers::delete_script))
        .route("/rooms", get(handlers::list_rooms).post(handlers::create_room))
        .route("/rooms/:id", get(handlers::get_room).put(handlers::update_room).delete(handlers::delete_room))
        .route("/sessions", get(handlers::list_sessions).post(handlers::create_session))
        .route("/sessions/:id", get(handlers::get_session).put(handlers::update_session).delete(handlers::delete_session))
        .route("/bookings", get(handlers::list_bookings).post(handlers::create_booking))
        .route("/bookings/:id", get(handlers::get_booking))
        .route("/bookings/:id/cancel", post(handlers::cancel_booking))
        .layer(cors)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
