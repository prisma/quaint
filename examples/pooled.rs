use quaint::pooled::{PooledConnection, Quaint};
use std::time::Instant;

#[tokio::main]
async fn main() -> () {
    let url = "DATABASE_URL".to_string();

    pooled_connection(&url).await;
}

async fn pooled_connection(url: &str) -> () {
    let now_total = Instant::now();
    let now = Instant::now();
    let quaint = build_quaint(&url);
    let elapsed = now.elapsed();
    println!("Quaint building: {:?}", elapsed);

    let now = Instant::now();
    let _ = get_conn(&quaint).await;
    println!("Conn acquired: {:?}", now.elapsed());

    println!("Total time: {:?}", now_total.elapsed());
}

async fn get_conn(quaint: &Quaint) -> PooledConnection {
    quaint.check_out().await.unwrap()
}

fn build_quaint(postgres_url: &str) -> Quaint {
    let mut builder = Quaint::builder(postgres_url).expect("should connect");

    builder.health_check_interval(std::time::Duration::from_secs(15));
    builder.test_on_check_out(true);

    builder.build()
}
