use mongodb::Database;
use warp::Filter;

pub mod galaxy;

use crate::db::get_db;
use crate::models::galaxy::Galaxy;

pub async fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let db_result = get_db("redmob").await;

    let database = db_result.unwrap();

    // GET /
    let base = warp::path::end().map(|| "Redmob!");

    // GET /status
    let status = warp::path!("status").map(|| "OK");

    let api = warp::path!("api" / ..);

    let api = api.and(galaxy::galaxies(database).or(warp::path!("status").map(|| "OK")));

    let routes = base.or(status).or(api);

    routes
}

fn with_db(
    db: Database,
) -> impl Filter<Extract = (mongodb::Database,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (Galaxy,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
