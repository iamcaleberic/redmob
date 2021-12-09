use mongodb::{bson::doc, options::ClientOptions, Client, bson::Document, Database, Collection};

use crate::{redis_cli::create_client, models::galaxy::Galaxy};

pub async fn create_db () -> Result<Client, mongodb::error::Error> {
    
    // Parse your connection string into an options struct
    let mut client_options =
        ClientOptions::parse("mongodb://admin:test@0.0.0.0:27017")
            .await?;
    // Manually set an option
    client_options.app_name = Some("redmob".to_string());
    // Get a handle to the cluster
    let client: mongodb::Client = Client::with_options(client_options)?;

    info!("Attemp db connect.");

    // Ping the server to see if you can connect to the cluster
    client
        .database("redmob")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    info!("Connected successfully.");

    // // Get a handle to a database.
    // let db = client.database("redmob");

    // // List the names of the collections in that database.
    // for collection_name in db.list_collection_names(None).await? {
    //     info!("c: {}", collection_name);
    // }

    // // Get a handle to a collection in the database.
    // let collection = db.collection::<Document>("books");

    // let docs = vec![
    //     doc! { "title": "1984", "author": "George Orwell" },
    //     doc! { "title": "Animal Farm", "author": "George Orwell" },
    //     doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    // ];

    // // Insert some documents into the "mydb.books" collection.
    // collection.insert_many(docs, None).await?;

    Ok(client) 
}

pub async fn get_db(name: &str) -> Result<Database, mongodb::error::Error> {
    let client = create_db().await;
    match client {
        Ok(client) => {
            return Ok(client.database(name));
        }

        Err(err) => Err(err)
        
    }
}   

pub async fn get_galaxy_collection(name: &str, db: Database) -> Collection<Galaxy> {
    let collection = db.collection::<Galaxy>(name);
    collection
}