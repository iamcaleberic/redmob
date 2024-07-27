use futures::stream::StreamExt;
use mongodb::{bson::doc, Database};
use warp::http::StatusCode;
// use std::collections::HashMap;
use redis::AsyncCommands;
// use warp::reply::Reply;

use crate::models::galaxy::Galaxy;
use crate::redis_cli;
// use crate::routes::galaxy::galaxies;
use crate::db;

use std::convert::Infallible;

const COLLECTION: &str = "galaxies";

pub async fn create_galaxy(galaxy: Galaxy, db: Database) -> Result<impl warp::Reply, Infallible> {
    debug!("create galaxy");

    info!("{:?}", galaxy);

    // Get a handle to a collection in the database.
    let collection = db::get_galaxy_collection(COLLECTION, db).await;

    let res = collection.insert_one(galaxy).await;

    match res {
        Ok(res) => {
            debug!("result: {:?}", res);
            Ok(StatusCode::CREATED)
        }
        Err(err) => {
            debug!("error inserting doc {:?}", err);
            return Ok(StatusCode::BAD_REQUEST);
        }
    }
}

pub async fn get_galaxies(db: Database) -> Result<impl warp::Reply, Infallible> {
    debug!("get galaxies");

    // Get a handle to a collection in the database.
    let collection = db::get_galaxy_collection(COLLECTION, db).await;
    // let find_options = mongodb::options::FindOptions::builder().build();
    let mut cursor = collection.find(doc! {}).await.unwrap();

    let mut result: Vec<Galaxy> = Vec::new();

    while let Some(Ok(doc)) = cursor.next().await {
        result.push(doc)
    }

    debug!("result: {:?}", result);

    Ok(warp::reply::json(&result))
}

pub async fn get_galaxy_by_oid(oid: String, db: Database) -> Result<impl warp::Reply, Infallible> {
    debug!("get galaxy by oid");

    let parsed_oid = mongodb::bson::oid::ObjectId::parse_str(&oid).unwrap();
    let filter = doc! { "_id": parsed_oid  };

    let collection = db::get_galaxy_collection(COLLECTION, db).await;
    // // let find_options = mongodb::options::FindOptions::builder().build();
    // let mut cursor = collection.find(filter, None).await.unwrap();

    // let mut result: Vec<Galaxy> = Vec::new();

    // while let Some(Ok(doc)) = cursor.next().await {
    //     result.push(doc)
    // }

    let rc = redis_cli::create_client().await;
    match rc {
        Ok(conn) => {
            let mut conn_cli = conn;
            let redis_key: &String = &oid;

            let cached_galaxy: redis::RedisResult<redis::Value> = conn_cli.get(redis_key).await;

            match cached_galaxy {
                Ok(redis::Value::Nil) => {
                    // let find_options = mongodb::options::FindOptions::builder().build();
                    let result = collection.find_one(filter).await.unwrap();

                    if let Some(glxy) = &result {
                        let _: Result<(), redis::RedisError> = redis::pipe()
                            .atomic()
                            .set(&oid, &glxy)
                            .expire(&oid, 60)
                            .query_async(&mut conn_cli)
                            .await;
                    }

                    debug!("{:?}", &result);
                    return Ok(warp::reply::json(&result));
                }

                Ok(redis::Value::Data(val)) => {
                    let actual: Galaxy = serde_json::from_slice(&val).unwrap();
                    return Ok(warp::reply::json(&actual));
                }

                _ => {
                    debug!("unexpected value");
                }
            }
        }

        Err(err) => {
            debug!("redis error , {:?}", err);
        }
    }

    Ok(warp::reply::json(&""))
}
