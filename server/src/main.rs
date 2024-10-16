use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

use actix_cors::Cors;
use actix_middleware_etag::Etag;
use actix_web::web::Redirect;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
use meilisearch_sdk::client::Client;
use sentry::SessionMode;
use sqlx::postgres::PgPoolOptions;
use sqlx::prelude::*;
use sqlx::{PgPool, Pool, Postgres};
use tokio::sync::{Barrier, RwLock};
use tracing::{debug_span, error, info};
use tracing_actix_web::TracingLogger;

mod calendar;
mod feedback;
mod limited;
mod localisation;
mod locations;
mod maps;
mod models;
mod search;
mod setup;

const MAX_JSON_PAYLOAD: usize = 1024 * 1024; // 1 MB

#[derive(Clone, Debug)]
pub struct AppData {
    /// shared [sqlx::PgPool] to connect to postgis
    pool: PgPool,
    /// necessary, as otherwise we could return empty results during initialisation
    meilisearch_initialised: Arc<RwLock<()>>,
}

impl AppData {
    async fn new() -> Self {
        let pool = PgPoolOptions::new()
            .min_connections(2)
            .connect(&connection_string())
            .await
            .expect("make sure that postgis is running in the background");
        AppData {
            pool,
            meilisearch_initialised: Arc::new(Default::default()),
        }
    }
}

#[get("/api/status")]
async fn health_status_handler(data: web::Data<AppData>) -> HttpResponse {
    let github_link = match option_env!("GIT_COMMIT_SHA") {
        Some(hash) => format!("https://github.com/TUM-Dev/navigatum/tree/{hash}"),
        None => "unknown commit hash, probably running in development".to_string(),
    };
    match data.pool.execute("SELECT 1").await {
        Ok(_) => HttpResponse::Ok()
            .content_type("text/plain")
            .body(format!("healthy\nsource_code: {github_link}")),
        Err(e) => {
            error!("database error: {e:?}",);
            HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body(format!("unhealthy\nsource_code: {github_link}"))
        }
    }
}
#[get("/api/get/{id}")]
async fn details_redirect(params: web::Path<String>) -> impl Responder {
    let id = params.into_inner();
    Redirect::to(format!("https://nav.tum.de/locations/{id}")).permanent()
}
#[get("/api/preview/{id}")]
async fn preview_redirect(params: web::Path<String>) -> impl Responder {
    let id = params.into_inner();
    Redirect::to(format!("https://nav.tum.de/locations/{id}/preview")).permanent()
}

fn connection_string() -> String {
    let username = std::env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string());
    let password = std::env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "CHANGE_ME".to_string());
    let url = std::env::var("POSTGRES_URL").unwrap_or_else(|_| "localhost".to_string());
    let db = std::env::var("POSTGRES_DB").unwrap_or_else(|_| username.clone());
    format!("postgres://{username}:{password}@{url}/{db}")
}

pub fn setup_logging() {
    use tracing_subscriber::filter::EnvFilter;
    use tracing_subscriber::fmt::Layer;
    use tracing_subscriber::prelude::*;
    let default_level = if cfg!(any(debug_assertions, test)) {
        "debug"
    } else {
        "info"
    };
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| default_level.to_string());
    // these overrides exist to filter away stuff I don't think we should ever care about
    let filter = format!("{log_level},hyper=info,rustls=info,h2=info,sqlx=info,hickory_resolver=info,hickory_proto=info");

    let filter = EnvFilter::builder().parse_lossy(filter);

    tracing_log::LogTracer::builder()
        .with_interest_cache(tracing_log::InterestCacheConfig::default())
        .init()
        .expect("the global logger to only be set once");

    let registry = tracing_subscriber::registry()
        .with(filter)
        .with(sentry::integrations::tracing::layer())
        .with(cfg!(not(any(debug_assertions, test))).then(|| Layer::default().json()))
        .with(cfg!(any(debug_assertions, test)).then(|| Layer::default().pretty()));
    tracing::subscriber::set_global_default(registry).unwrap();
}

fn main() -> anyhow::Result<()> {
    setup_logging();
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("no provider was set as default beforehand");
    let release = match option_env!("GIT_COMMIT_SHA") {
        Some(s) => Some(Cow::Borrowed(s)),
        None => sentry::release_name!(),
    };
    let _guard = sentry::init((
        "https://8f2054d6294447a1b573ea4badb76778@sentry.mm.rbg.tum.de/8",
        sentry::ClientOptions {
            release,
            traces_sample_rate: 1.0,
            session_mode: SessionMode::Request,
            auto_session_tracking: true,
            ..Default::default()
        },
    ));
    std::env::set_var("RUST_BACKTRACE", "1");

    actix_web::rt::System::new().block_on(async { run().await })?;
    Ok(())
}
async fn run_maintenance_work(
    pool: Pool<Postgres>,
    meilisearch_initialised: Arc<RwLock<()>>,
    initialisation_started: Arc<Barrier>,
) {
    if std::env::var("SKIP_MS_SETUP") != Ok("true".to_string()) {
        let _ = debug_span!("updating meilisearch data").enter();
        let _ = meilisearch_initialised.write().await;
        initialisation_started.wait().await;
        let ms_url =
            std::env::var("MIELI_URL").unwrap_or_else(|_| "http://localhost:7700".to_string());
        let client = Client::new(ms_url, std::env::var("MEILI_MASTER_KEY").ok()).unwrap();
        setup::meilisearch::setup(&client, true).await.unwrap();
        setup::meilisearch::load_data(&client, true).await.unwrap();
    } else {
        info!("skipping the database setup as SKIP_MS_SETUP=true");
        initialisation_started.wait().await;
    }
    if std::env::var("SKIP_DB_SETUP") != Ok("true".to_string()) {
        let _ = debug_span!("updating postgis data").enter();
        setup::database::setup(&pool).await.unwrap();
        setup::database::load_data(&pool).await.unwrap();
        setup::transportation::setup(&pool).await.unwrap();
    } else {
        info!("skipping the database setup as SKIP_DB_SETUP=true");
    }
    let mut set = tokio::task::JoinSet::new();
    let map_pool = pool.clone();
    set.spawn(async move { maps::refresh::all_entries(&map_pool).await });
    let cal_pool = pool.clone();
    set.spawn(async move { calendar::refresh::all_entries(&cal_pool).await });
    while set.join_next().await.is_some() {}
}

/// we split main and run because otherwise sentry could not be properly instrumented
async fn run() -> anyhow::Result<()> {
    let data = AppData::new().await;

    // without this barrier an external client might race the RWLock for meilisearch_initialised and gain the read lock before it is allowed
    let initialisation_started = Arc::new(Barrier::new(2));
    let maintenance_thread = tokio::spawn(run_maintenance_work(
        data.pool.clone(),
        data.meilisearch_initialised.clone(),
        initialisation_started.clone(),
    ));

    let prometheus = build_metrics();
    let shutdown_pool_clone = data.pool.clone();
    initialisation_started.wait().await;
    info!("running the server");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600)
            .send_wildcard();

        App::new()
            .wrap(Etag)
            .wrap(prometheus.clone())
            .wrap(cors)
            .wrap(TracingLogger::default())
            .wrap(middleware::Compress::default())
            .wrap(sentry_actix::Sentry::new())
            .app_data(web::JsonConfig::default().limit(MAX_JSON_PAYLOAD))
            .app_data(web::Data::new(data.clone()))
            .service(health_status_handler)
            .service(calendar::calendar_handler)
            .service(maps::indoor::list_indoor_maps)
            .service(maps::indoor::get_indoor_map)
            .service(search::search_handler)
            .service(web::scope("/api/feedback").configure(feedback::configure))
            .service(web::scope("/api/locations").configure(locations::configure))
            .service(details_redirect)
            .service(preview_redirect)
    })
    .bind(std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3003".to_string()))?
    .run()
    .await?;
    maintenance_thread.abort();
    shutdown_pool_clone.close().await;
    Ok(())
}

#[tracing::instrument]
fn build_metrics() -> PrometheusMetrics {
    let labels = HashMap::from([(
        "revision".to_string(),
        option_env!("GIT_COMMIT_SHA")
            .unwrap_or_else(|| "development")
            .to_string(),
    )]);
    PrometheusMetricsBuilder::new("navigatum_api")
        .endpoint("/api/metrics")
        .const_labels(labels)
        .build()
        .expect("specified metrics are valid")
}
