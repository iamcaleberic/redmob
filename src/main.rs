use pretty_env_logger;
use redis::AsyncCommands;

#[macro_use]
extern crate log;

mod db;
mod models;
mod redis_cli;
mod routes;
mod handlers;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("redmob");

    // let rc = redis_cli::create_client().await;
    // match rc {
    //     Ok(conn) => {
    //         let mut conn_cli = conn;
    //         let _rs: redis::RedisResult<()> = conn_cli.set("key1", b"wow").await;
    //     }

    //     Err(err) => {
    //         println!("err , {:?}", err)
    //     }
    // }

    // let mc = db::create_db().await;

    // match mc {
    //     Ok(_m_cli) => {}
    //     Err(err) => {
    //         println!("err , {:?}", err)
    //     }
    // }

    warp::serve(routes::routes().await)
        .run(([0, 0, 0, 0], 3030))
        .await;
}
