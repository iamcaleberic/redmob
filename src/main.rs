use pretty_env_logger;

#[macro_use]
extern crate log;

mod db;
mod handlers;
mod models;
mod redis_cli;
mod routes;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("redmob");

    warp::serve(routes::routes().await)
        .run(([0, 0, 0, 0], 3030))
        .await;
}
