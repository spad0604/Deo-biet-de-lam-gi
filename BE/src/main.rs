mod db;
mod models;
mod api;
mod security;
mod services;
mod entity;
mod config;
mod utils;

use std::net::SocketAddr;
use sqlx::PgPool;
use tracing_subscriber;
use axum::{Router, serve};
use crate::api::{user_api, auth_api, internal_api};
use crate::config::Config;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tokio::net::TcpListener;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::user_api::get_me,
        crate::api::user_api::upload_avatar,
        crate::api::auth_api::register,
        crate::api::auth_api::login,
    ),
    components(
        schemas(
            crate::models::login_response::LoginResponse,
            crate::models::register_request::RegisterRequest,
            crate::models::register_request::LoginRequest,
            crate::entity::users::Role,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Authentication", description = "Authentication endpoints"),
        (name = "User", description = "User management endpoints")
    )
)]
struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        use utoipa::openapi::security::{SecurityScheme, HttpAuthScheme, HttpBuilder};
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build()
                ),
            )
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	// Load .env file if exists
	dotenvy::dotenv().ok();
	
	tracing_subscriber::fmt::init();

	let cfg = Config::from_env();
	let _database_url = cfg.database_url();
	let addr: SocketAddr = cfg.addr.parse()?;

	let pool = crate::db::db::connect_db().await?;

	let app = Router::new()
		.merge(user_api::routes())
		.merge(auth_api::routes())
		.merge(internal_api::routes())
		.merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
		.with_state(pool.clone());

	tracing::info!("🚀 Server running at http://{}", addr);

	let listener = TcpListener::bind(addr).await?;
	serve(listener, app).await?;

	Ok(())
}
