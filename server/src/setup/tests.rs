use meilisearch_sdk::client::Client;
use testcontainers_modules::testcontainers::{ContainerAsync, ImageExt};
use testcontainers_modules::{meilisearch, testcontainers::runners::AsyncRunner};

pub struct PostgresTestContainer {
    _container: ContainerAsync<testcontainers_modules::postgres::Postgres>,
    pub pool: sqlx::Pool<sqlx::Postgres>,
}

impl PostgresTestContainer {
    /// Create a postgres instance for testing against
    pub async fn new() -> Self {
        let container = testcontainers_modules::postgres::Postgres::default()
            .with_tag("16-3.4")
            .with_name("postgis/postgis")
            .start()
            .await
            .unwrap();
        let connection_string = format!(
            "postgres://postgres:postgres@{host}:{port}/postgres",
            host = container.get_host().await.unwrap(),
            port = container.get_host_port_ipv4(5432).await.unwrap(),
        );
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(&connection_string)
            .await
            .unwrap();
        crate::setup::database::setup(&pool).await.unwrap();
        Self {
            _container: container,
            pool,
        }
    }
}

pub struct MeiliSearchTestContainer {
    _container: ContainerAsync<meilisearch::Meilisearch>,
    pub client: Client,
}

impl MeiliSearchTestContainer {
    /// Create a meilisearch instance for testing against
    pub async fn new(vector_search: bool) -> Self {
        let container = meilisearch::Meilisearch::default()
            .with_tag("v1.9.0")
            .start()
            .await
            .unwrap();
        let meili_url = format!(
            "http://{host}:{port}",
            host = container.get_host().await.unwrap(),
            port = container.get_host_port_ipv4(7700).await.unwrap(),
        );

        let client = Client::new(meili_url.clone(), None::<String>).unwrap();
        super::meilisearch::setup(&client, vector_search)
            .await
            .unwrap();
        Self {
            _container: container,
            client,
        }
    }
}

#[tokio::test]
#[tracing_test::traced_test]
async fn test_db_setup() {
    let pg = PostgresTestContainer::new().await;
    for i in 0..20 {
        let res = crate::setup::database::load_data(&pg.pool).await;
        if let Err(e) = res {
            tracing::error!("failed to load db because {e:?}. Retrying for 20s");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        } else {
            tracing::info!("successfully initalised the db in try {i}");
            break;
        }
    }
}

#[tokio::test]
#[tracing_test::traced_test]
async fn test_meilisearch_setup() {
    let ms = MeiliSearchTestContainer::new(false).await;
    crate::setup::meilisearch::load_data(&ms.client, false)
        .await
        .unwrap();
}
