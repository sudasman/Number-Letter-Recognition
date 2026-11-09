use axum::{
    Json, Router,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;


#[derive(Debug)]
pub enum ApiError {
    NotFound,
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message): (StatusCode, String) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, String::from("no such path")),
            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("parsing error"),
            ),
        };

        let body = Json(json!({"Error": error_message }));

        (status, body).into_response()
    }
}

pub async fn new() -> Result<(), ApiError> {
    let router = Router::new()
        .route("/", get(home).post(post))
        .route("/{*rest}", get(not_found));

    let address: String = String::from("0.0.0.0:3000");

    let listener: TcpListener = TcpListener::bind(address)
        .await
        .expect("couldnt bind to address");

    axum::serve(listener, router)
        .await
        .expect("unable to start web server");

    Ok(())
}

async fn home() {}

async fn post() {}

async fn not_found() -> ApiError {
    ApiError::NotFound
}
