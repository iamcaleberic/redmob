use redis::RedisError;
use redis::aio::ConnectionManager;
use std::env;

pub async fn create_client() -> Result<ConnectionManager, RedisError> {
    let url = if env::args().nth(1) == Some("--tls".into()) {
        "rediss://127.0.0.1:6380/#insecure"
    } else {
        "redis://127.0.0.1:6379/"
    };

    let client = redis::Client::open(url)?;
    let conn = client.get_tokio_connection_manager().await?;
    Ok(conn)
}

