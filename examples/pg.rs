use futures::FutureExt;
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use quaint::connector::PostgresUrl;
use tokio;

#[tokio::main]
async fn main() -> () {
    let url = "DATABASE_URL".to_string();
    let url = PostgresUrl::new(url::Url::parse(&url).unwrap()).unwrap();

    connect_tls(url).await
}

async fn connect_tls(url: PostgresUrl) -> () {
    let config: tokio_postgres::Config = url.to_config();

    let mut tls_builder = TlsConnector::builder();
    tls_builder.danger_accept_invalid_certs(true);

    let tls = MakeTlsConnector::new(tls_builder.build().unwrap());

    let now = std::time::Instant::now();
    let (_, conn) = config.connect(tls).await.unwrap();
    println!("conn: {:?}", now.elapsed());

    tokio::spawn(conn.map(|r| match r {
        Ok(_) => (),
        Err(e) => {
            tracing::error!("Error in PostgreSQL connection: {:?}", e);
        }
    }));
}
