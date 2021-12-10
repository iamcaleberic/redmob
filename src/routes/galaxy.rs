use mongodb::Database;
use warp::Filter;

use super::{json_body, with_db};
use crate::handlers::galaxy::{create_galaxy, get_galaxies, get_galaxy_by_oid};

// use std::collections::HashMap;

pub fn galaxies(
    db: Database,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    create(db.clone())
        .or(get_by_oid(db.clone()))
        .or(get(db.clone()))
}

/// GET /galaxies
pub fn get(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("galaxies")
        .and(warp::get())
        .and(with_db(db))
        .and_then(get_galaxies)
}

/// GET /galaxies/<iod>
pub fn get_by_oid(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("galaxies" / String)
        .and(warp::get())
        // .and(warp::query::<HashMap<String, String>>())
        .and(with_db(db))
        .and_then(get_galaxy_by_oid)
}

// POST /galaxies/
pub fn create(
    db: Database,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("galaxies")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(create_galaxy)
}
