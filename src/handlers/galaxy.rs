use futures::stream::StreamExt;
use mongodb::{bson::doc, bson::Document, Collection, Database};
use warp::http::StatusCode;
use std::collections::HashMap;
use redis::AsyncCommands;

use crate::models::galaxy::Galaxy;
use crate::redis_cli;
use std::convert::Infallible;

pub async fn create_galaxy(galaxy: Galaxy, db: Database) -> Result<impl warp::Reply, Infallible> {
    debug!("create galaxy");

    info!("{:?}", galaxy);

    // Get a handle to a collection in the database.
    let collection = db.collection::<Galaxy>("galaxies");

    let docs = vec![galaxy];

    let res = collection.insert_many(docs, None).await;

    match res {
        Ok(res) => {
            debug!("result: {:?}", res);
            Ok(StatusCode::CREATED)
        }
        Err(err) => {
            debug!("error insterting doc {:?}", err);
            return Ok(StatusCode::BAD_REQUEST);
        }
    }
}

pub async fn get_galaxies(db: Database) -> Result<impl warp::Reply, Infallible> {
    debug!("get galaxies");

    // Get a handle to a collection in the database.
    let collection = db.collection::<Galaxy>("galaxies");
    // let find_options = mongodb::options::FindOptions::builder().build();
    let mut cursor = collection.find(None, None).await.unwrap();

    let mut result: Vec<Galaxy> = Vec::new();

    while let Some(Ok(doc)) = cursor.next().await {
        result.push(doc)
    }

    debug!("result: {:?}", result);

    Ok(warp::reply::json(&result))
}



pub async fn get_galaxy_by_name(kv: HashMap<String, String>, db: Database) -> Result<impl warp::Reply, Infallible> {
    debug!("get galaxy by name");

    let filter = doc! { "name": kv.get("name") };
    
    let collection = db.collection::<Galaxy>("galaxies");
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
            let redis_key: &String = kv.get("name").unwrap();

            let cached_galaxy: redis::RedisResult<redis::Value> = conn_cli.get(redis_key).await;

            match cached_galaxy {
                Ok(redis::Value::Nil) => {
                        // let find_options = mongodb::options::FindOptions::builder().build();
                    let mut cursor = collection.find(filter, None).await.unwrap();

                    let mut result: Vec<Galaxy> = Vec::new();

                    while let Some(Ok(doc)) = cursor.next().await {
                        result.push(doc)
                    }

                    debug!("result: {:?}", result);

                    for res in result.clone() {

                        let _:  Result<(), redis::RedisError>= redis::pipe()
                            .atomic()
                            .set(&redis_key, &res)
                            .expire(&redis_key, 60)
                            .query_async(&mut conn_cli)
                            .await;
                    }

                    return Ok(warp::reply::json(&result))

                }

                Ok(redis::Value::Data(val)) =>{
                    return Ok(warp::reply::json(&val))
                }

                _ => {
                    debug!("unexpected value");
                    
                }
            }
        }

        Err(err) => {
            println!("redis error , {:?}", err)
        }
    }



    Ok(warp::reply::json(&""))
}

