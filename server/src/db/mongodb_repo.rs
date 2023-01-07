use crate::{models::santa_model::Room, models::santa_model::StatusRoom};

use mongodb::{
    bson::{doc, oid::ObjectId},
    options::ClientOptions,
    Client, Collection,
};

use rand::{seq::SliceRandom, thread_rng};
use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid ObjectId")]
    ObjectIdError(#[from] mongodb::bson::oid::Error),
    #[error("MongoDB Error")]
    MongoDBError(#[from] mongodb::error::Error),
    #[error("Room not found")]
    TaskNotFound,
    #[error("Unexpected error")]
    UnexpectedError,
    #[error("Game has not been started yet")]
    GameNotStarted,
    #[error("Game has been already started")]
    GameStarted,
    #[error("Game has been already ended")]
    GameEnded,
    #[error("Incorrect admin token")]
    IncorrectAdminToken,
    #[error("You cannot start game alone")]
    CannotStartAlone,
}

pub struct MongoRepo {
    santas: Collection<Room>,
}

impl MongoRepo {
    pub async fn init(uri: &str) -> Result<Self, Error> {
        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database("santa");
        let santas = db.collection::<Room>("santas");
        Ok(MongoRepo { santas })
    }
}
