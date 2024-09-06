use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use env_logger::Env;
use git_oidc::{fetch_jwks, GithubJWKS};
use log::{debug, error, info};  
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;

struct AppState {
    jwks: Arc<RwLock<GithubJWKS>>,
}

#[derive(Deserialize)]
struct TokenRequest {
    token: String,
}


async fn token_endpoint(
    token_request: web::Json<TokenRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    debug!("Received token validation request");
    match GithubJWKS::validate_github_token(
        &token_request.token,
        data.jwks.clone(),
        Some("https://github.com/Seif-Mamdouh"),
    )
    .await
    {
        Ok(claims) => {
            info!("Token validated successfully");
            HttpResponse::Ok().json(claims)
        }
        Err(e) => {
            error!("Token validation error: {:?}", e);
            HttpResponse::BadRequest().body(format!("Invalid token: {}", e))
        }
    }
}

async fn hello() -> impl Responder {
    "Hello, OIDC!"
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    // Remove color_eyre::install()?;

    info!("Starting OIDC server...");

    let github_oidc_url = "https://token.actions.githubusercontent.com";
    let jwks = Arc::new(RwLock::new(fetch_jwks(github_oidc_url).await?));

    if let Ok(org) = std::env::var("GITHUB_ORG") {
        info!("GITHUB_ORG set to: {}", org);
    }
    if let Ok(repo) = std::env::var("GITHUB_REPO") {
        info!("GITHUB_REPO set to: {}", repo);
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { jwks: jwks.clone() }))
            .route("/", web::get().to(hello))
            .route("/token", web::post().to(token_endpoint))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await?;

    Ok(())
}
