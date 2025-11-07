mod db;
mod models;
mod api;
mod security;
mod services;
mod entity;
mod config;

use std::net::SocketAddr;
use sqlx::PgPool;
use tracing_subscriber;
use axum::{Router, serve};
use crate::api::user_api;
use crate::config::Config;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tokio::net::TcpListener;

#[derive(OpenApi)]
#[openapi(paths())]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::fmt::init();

	let cfg = Config::from_env();
	let database_url = cfg.database_url();
	let addr: SocketAddr = cfg.addr.parse()?;

	let pool = crate::db::db::connect_db().await?;

	let app = Router::new()
		.merge(user_api::routes())
		.merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
		.with_state(pool.clone());

	tracing::info!("🚀 Server running at http://{}", addr);

	let listener = TcpListener::bind(addr).await?;
	serve(listener, app).await?;

	Ok(())
}
