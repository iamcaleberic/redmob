use warp::Filter;
use redis::{AsyncCommands};
use pretty_env_logger;

#[macro_use] extern crate log;

mod db;
mod routes;
mod redis_cli;

#[tokio::main]
async fn main() {

    pretty_env_logger::init();
    info!("redmob");
    
    let rc = redis_cli::create_client().await;
    match rc {
        Ok(conn) => {
            let mut conn_cli = conn;
            let _rs:redis::RedisResult<()> = conn_cli.set("key1", b"wow").await;
        }
        
        Err(err) => {println!("err , {:?}", err)}
    }

    
    // GET /
    let base = warp::path::end().map(|| 
        "Redmob!"
    );

    // GET /status
    let status = warp::path!("status").map(|| "OK");

    let routes = warp::get().and(
        base
        .or(status)
    );
    
    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;
}
 