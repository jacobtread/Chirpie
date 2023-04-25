use actix_web::{
    get,
    http::StatusCode,
    web::{Json, Path},
    ResponseError,
};
use database::{Chirp, User};
use dotenvy::dotenv;
use thiserror::Error;

mod database;

#[tokio::main]
async fn main() {
    // Load environment variables
    let _ = dotenv();

    println!("Hello, world!");
}

#[derive(Debug, Error)]
pub enum ProfileError {
    #[error("Profile not found")]
    UnknownProfile,
}

type ProfileRes<T> = Result<Json<T>, ProfileError>;

#[get("/api/profiles/{name}")]
pub async fn get_user_profile(path: Path<String>) -> ProfileRes<User> {
    let name = path.into_inner();
    Err(ProfileError::UnknownProfile)
}

#[derive(Debug, Error)]
pub enum ChirpError {
    #[error("Profile not found")]
    UnknownProfile,
}

type ChirpRes<T> = Result<Json<T>, ChirpError>;

#[get("/api/profiles/{user_id}")]
pub async fn get_all_chirps() -> ChirpRes<Vec<Chirp>> {
    Err(ChirpError::UnknownProfile)
}

#[get("/api/chirps/{user_id}")]
pub async fn get_user_chirps(path: Path<String>) -> ChirpRes<Vec<Chirp>> {
    let user_id = path.into_inner();
    Err(ChirpError::UnknownProfile)
}

#[get("/api/chirps/{user_id}/{chirp_id}")]
pub async fn get_user_chirp(path: Path<(String, String)>) -> ChirpRes<Chirp> {
    let (user_id, chirp_id) = path.into_inner();
    Err(ChirpError::UnknownProfile)
}

impl ResponseError for ProfileError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::UnknownProfile => StatusCode::BAD_REQUEST,
        }
    }
}
impl ResponseError for ChirpError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::UnknownProfile => StatusCode::BAD_REQUEST,
        }
    }
}
