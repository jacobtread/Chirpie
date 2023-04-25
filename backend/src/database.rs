use std::process::exit;

use chrono::NaiveDateTime;
use log::error;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::{ClientOptions, FindOptions},
    Client, Database,
};
use serde::{Deserialize, Serialize};

/// Structure of a user within the app database
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// Unique ID for the user
    pub id: ObjectId,
    /// Unique username of the user
    pub username: String,
    /// Email address of the user
    pub email: String,
}

type DbResult<T> = mongodb::error::Result<T>;

impl User {
    const COLLECTION: &str = "users";

    pub async fn get_username(db: &Database, username: &str) -> DbResult<Option<Self>> {
        let collection = db.collection::<User>(Self::COLLECTION);
        let filter = doc! { "username": username };
        let user = collection.find_one(filter, None).await?;
        Ok(user)
    }

    /// Finds or creates a user where the email matches the provided
    /// email address
    ///
    /// `email` The email to use
    pub async fn get_email(db: &Database, email: &str) -> DbResult<Option<Self>> {
        let collection = db.collection::<User>(Self::COLLECTION);
        let filter = doc! { "email": email };
        let user = collection.find_one(filter, None).await?;
        Ok(user)
    }
}

/// Structure of a chirp (Tweet) message
#[derive(Debug, Serialize, Deserialize)]
pub struct Chirp {
    /// ID of the user that made the chirp
    pub owner: ObjectId,
    /// The text of the chirp
    pub text: String,
    /// Creation date and time
    pub created_at: NaiveDateTime,
}

const DATABASE_NAME: &str = "chirpie";
const DATABASE_URL: &str = "DATABASE_URL";

/// Connects to the MongoDB database returning a database connection. If there
/// is an error during connection the process will exit with an error message1
pub async fn connect() -> Database {
    // Obtain database url
    let database_url = match std::env::var(DATABASE_URL) {
        Ok(value) => value,
        Err(_) => {
            error!(
                "Missing database url environment variable: {}",
                DATABASE_URL
            );
            exit(1);
        }
    };

    // Setup client options
    let mut options = match ClientOptions::parse(database_url).await {
        Ok(value) => value,
        Err(err) => {
            error!("Failed to parse database options: {}", err);
            exit(1);
        }
    };
    options.app_name = Some("Chirpie".to_string());

    // Create client
    let client = match Client::with_options(options) {
        Ok(value) => value,
        Err(err) => {
            error!("Failed to connect to database: {}", err);
            exit(1);
        }
    };

    client.database(DATABASE_NAME)
}
