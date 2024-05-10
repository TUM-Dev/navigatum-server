use std::collections::HashMap;
use std::error::Error;

use actix_cors::Cors;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer};
use actix_web_prom::PrometheusMetricsBuilder;
use log::{debug, error, info};
use meilisearch_sdk::client::Client;
use sqlx::postgres::PgPoolOptions;
use sqlx::prelude::*;
use sqlx::PgPool;

mod calendar;
mod details;
mod feedback;
mod maps;
mod models;
mod search;
mod setup;
mod utils;

type BoxedError = Box<dyn Error + Send + Sync>;

const MAX_JSON_PAYLOAD: usize = 1024 * 1024; // 1 MB

#[derive(Debug)]
pub struct AppData {
    db: PgPool,
}

#[get("/api/status")]
async fn health_status_handler(data: web::Data<AppData>) -> HttpResponse {
    let github_link = match std::env::var("GIT_COMMIT_SHA") {
        Ok(hash) => format!("https://github.com/TUM-Dev/navigatum/tree/{hash}"),
        Err(_) => "unknown commit hash, probably running in development".to_string(),
    };
    return match data.db.execute("SELECT 1").await {
        Ok(_) => HttpResponse::Ok()
            .content_type("text/plain")
            .body(format!("healthy\nsource_code: {github_link}")),
        Err(e) => {
            error!("database error: {e:?}",);
            HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body(format!("unhealthy\nsource_code: {github_link}"))
        }
    };
}

fn connection_string() -> String {
    let username = std::env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string());
    let password = std::env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "password".to_string());
    let url = std::env::var("POSTGRES_URL").unwrap_or_else(|_| "localhost".to_string());
    let db = std::env::var("POSTGRES_DB").unwrap_or_else(|_| username.clone());
    format!("postgres://{username}:{password}@{url}/{db}")
}

pub fn setup_logging() {
    #[cfg(any(debug_assertions, test))]
    {
        let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "debug".to_string());
        let filter = format!("{log_level},hyper=info,rustls=info,h2=info,sqlx=info,hickory_resolver=info,hickory_proto=info");
        let env = env_logger::Env::default().default_filter_or(&filter);
        env_logger::Builder::from_env(env)
            .is_test(cfg!(test))
            .init();
    }
    #[cfg(not(any(debug_assertions, test)))]
    {
        let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        structured_logger::Builder::with_level(&log_level)
            .with_target_writer(
                "*",
                structured_logger::async_json::new_writer(tokio::io::stdout()),
            )
            .init();
    }
}

#[tokio::main]
async fn main() -> Result<(), BoxedError> {
    setup_logging();
    let maintenance_thread = tokio::spawn(async move {
        let pool = PgPoolOptions::new()
            .connect(&connection_string())
            .await
            .unwrap();
        #[cfg(not(feature = "skip_db_setup"))]
        {
            setup::database::setup(&pool).await.unwrap();
            setup::database::load_data(&pool).await.unwrap();
        }
        #[cfg(not(feature = "skip_ms_setup"))]
        {
            let ms_url =
                std::env::var("MIELI_URL").unwrap_or_else(|_| "http://localhost:7700".to_string());
            let client = Client::new(ms_url, std::env::var("MEILI_MASTER_KEY").ok()).unwrap();
            setup::meilisearch::setup(&client).await.unwrap();
            setup::meilisearch::load_data(&client).await.unwrap();
        }
        calendar::refresh::all_entries(&pool).await;
    });

    debug!("setting up metrics");
    let labels = HashMap::from([(
        "revision".to_string(),
        std::env::var("GIT_COMMIT_SHA").unwrap_or_else(|_| "development".to_string()),
    )]);
    let prometheus = PrometheusMetricsBuilder::new("navigatum_mainapi")
        .endpoint("/api/main/metrics")
        .const_labels(labels)
        .build()
        .unwrap();

    info!("running the server");
    let pool = PgPoolOptions::new().connect(&connection_string()).await?;
    let shutdown_pool_clone = pool.clone();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);

        App::new()
            .wrap(prometheus.clone())
            .wrap(cors)
            .wrap(middleware::Logger::default().exclude("/api/status"))
            .wrap(middleware::Compress::default())
            .app_data(web::JsonConfig::default().limit(MAX_JSON_PAYLOAD))
            .app_data(web::Data::new(AppData { db: pool.clone() }))
            .service(health_status_handler)
            .service(calendar::calendar_handler)
            .service(web::scope("/api/preview").configure(maps::configure))
            .service(web::scope("/api/feedback").configure(feedback::configure))
            .service(details::get_handler)
            .service(search::search_handler)
    })
    .bind(std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3003".to_string()))?
    .run()
    .await?;
    maintenance_thread.abort();
    shutdown_pool_clone.close().await;
    Ok(())
}
