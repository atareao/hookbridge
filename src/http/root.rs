use serde::Deserialize;
use std::sync::Arc;
use axum::{
    Router,
    routing,
    extract::{State, Path},
    response::{
        IntoResponse,
        Json
    },
};
use tracing::{info, debug};
use super::AppState;

#[derive(Debug, Deserialize)]
struct Body {
    message: String
}



pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/healthcheck",
            routing::get(healthcheck)
        )
        .route("/",
            routing::get(get_root)
        )
        .route("/:destination",
            routing::post(post_root)
        )
}


async fn get_root() -> impl IntoResponse{
    info!("get_root");
    Json(serde_json::json!({
        "status": "success",
        "message": "Up and running"
    }))
}

async fn post_root(
    Path(destination): Path<String>,
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<Body>
) -> impl IntoResponse{
    info!("post_root");
    debug!("Destination: {}", &destination);
    debug!("Payload: {:?}", &payload);
    match app_state.config.get_destination(&destination){
        Some(service) => {
            match service.post(&payload.message).await{
                Ok(message) => Json(serde_json::json!({
                    "status": "ok",
                    "message": "Mensaje enviado",
                    "content": message
                })),
                Err(error_message) => Json(serde_json::json!({
                    "status": "fail",
                    "message": error_message
                }))
            }

        },
            None => Json(serde_json::json!({
                "status": "fail",
                "message": "Destination not exists"
            }))
    }
}

async fn healthcheck() -> impl IntoResponse{
    Json(serde_json::json!({
        "status": "success",
        "message": "Up and running"
    }))
}

