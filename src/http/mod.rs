mod root;


use super::model::Configuration;
use tower_http::trace::TraceLayer;
use std::{sync::Arc, net::{SocketAddr, Ipv4Addr}};
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
    axum::Server::bind(
        &SocketAddr::new(
            std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            config.get_port()))
        .serve(app.into_make_service())
        .await
        .map_err(|_err| String::from("Can't init"))
}

fn api_router(app_state: AppState) -> Router {
        root::router()
            .with_state(Arc::new(app_state.clone()))
}
