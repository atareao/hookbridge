mod root;


use super::model::Configuration;
use tower_http::trace::TraceLayer;
use std::sync::Arc;
use tokio::net::TcpListener;
use axum::Router;

#[derive(Clone)]
pub struct AppState {
    pub config: Configuration,
}

pub async fn serve(config: Configuration) -> Result<(), String>{

    let app = api_router(
            AppState {
                config: config.clone(),
            })
            .layer(TraceLayer::new_for_http());
    let addr = format!("0.0.0.0:{}", config.get_port());
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|_err| String::from("Can't init listener"))?;
    axum::serve(listener, app)
        .await
        .map_err(|_err| String::from("Can't init"))

}

fn api_router(app_state: AppState) -> Router {
        root::router()
            .with_state(Arc::new(app_state.clone()))
}
