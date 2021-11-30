use redis::RedisError;
use redis::aio::Connection;
use std::env;

pub async fn create_client() -> Result<Connection, RedisError> {
    let url = if env::args().nth(1) == Some("--tls".into()) {
        "rediss://127.0.0.1:6380/#insecure"
    } else {
        "redis://127.0.0.1:6379/"
    };

    let client = redis::Client::open(url)?;
    let conn = client.get_async_connection().await?;
    
    Ok(conn)
}

