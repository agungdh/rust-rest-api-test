mod application;
mod config;
mod domain;
mod infrastructure;
mod presentation;

use std::sync::Arc;

use axum::Router;
use tracing_subscriber::EnvFilter;

use application::services::{EmployeeService, EmployeeServiceTrait};
use config::AppConfig;
use infrastructure::InMemoryEmployeeRepository;
use presentation::employee_routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = AppConfig::load().unwrap_or_default();

    let repository = Arc::new(InMemoryEmployeeRepository::new());
    let service = Arc::new(EmployeeService::new(repository)) as Arc<dyn EmployeeServiceTrait>;

    let app = employee_routes().with_state(service);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::info!("Server running on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
