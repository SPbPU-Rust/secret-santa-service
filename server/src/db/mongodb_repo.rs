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
    pub async fn create_room(&self, mut santa: Room) -> Result<ObjectId, Error> {
        santa.id = None;
        self.santas
            .insert_one(santa, None)
            .await?
            .inserted_id
            .as_object_id()
            .ok_or(Error::UnexpectedError)
    }
    pub async fn get_room(&self, id: &str, name: &str) -> Result<StatusRoom, Error> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": obj_id};
        let mut room = self
            .santas
            .find_one(filter.clone(), None)
            .await?
            .ok_or(Error::TaskNotFound);

        let mut game_status = room.as_mut().unwrap().game_status;
        if game_status.is_none() {
            game_status = Some(0);
        }
        let game_status = game_status.unwrap();

        let mut users = room.as_mut().unwrap().users.clone();
        if users.is_none() {
            users = Some(Vec::new());
        }
        let mut users: Vec<String> = users.unwrap();

        if game_status == 0 {
            if users.binary_search(&name.to_string()).is_ok() == true {
            } else {
                users.push(name.to_string());
                users.sort();
                self.santas
                    .update_one(filter.clone(), doc! {"$set": {"users": &users}}, None)
                    .await?;
            }
            return Ok(StatusRoom {
                my_santa: ("None, because game has not been started yet.".to_string()),
                my_kid: ("None, because game has not been started yet.".to_string()),
            });
        } else if game_status == 1 {
            if users.iter().find(|&x| x == &name.to_string()).is_none() == false {
                let index = users.iter().position(|x| x == &name.to_string()).unwrap();
                if users[index] == users[users.len() - 1] {
                    return Ok(StatusRoom {
                        my_santa: ("It's a secret until game ends.".to_string()),
                        my_kid: (users[0].clone()),
                    });
                } else {
                    return Ok(StatusRoom {
                        my_santa: ("It's a secret until game ends.".to_string()),
                        my_kid: (users[index + 1].clone()),
                    });
                }
            } else {
                return Err(Error::GameStarted);
            }
        } else {
            if users.iter().find(|&x| x == &name.to_string()).is_none() == false {
                let index = users.iter().position(|x| x == &name.to_string()).unwrap();
                if users[index] == users[0] {
                    return Ok(StatusRoom {
                        my_santa: (users[users.len() - 1].clone()),
                        my_kid: (users[1].clone()),
                    });
                } else if users[index] == users[users.len() - 1] {
                    return Ok(StatusRoom {
                        my_santa: (users[index - 1].clone()),
                        my_kid: (users[0].clone()),
                    });
                } else {
                    return Ok(StatusRoom {
                        my_santa: (users[index - 1].clone()),
                        my_kid: (users[index + 1].clone()),
                    });
                }
            } else {
                return Err(Error::GameEnded);
            }
        }
    }

    pub async fn start_game(&self, id: &str, admin_token: &str) -> Result<Room, Error> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": obj_id};
        let mut game: Result<Room, Error> = self
            .santas
            .find_one(filter.clone(), None)
            .await?
            .ok_or(Error::TaskNotFound);

        let mut game_status = game.as_ref().unwrap().game_status;
        if game_status.is_none() {
            game_status = Some(0);
        }
        let mut game_status = game_status.unwrap();

        let mut users = game.as_mut().unwrap().users.clone();
        if users.is_none() {
            users = Some(Vec::new());
        }
        let mut users = users.unwrap();

        if users.len() == 1 {
            return Err(Error::CannotStartAlone);
        } else {
            if game_status == 0 {
                let game_token = game.as_ref().unwrap().admin_token.clone();
                if admin_token == game_token {
                    game_status = 1;
                    let mut rng = thread_rng();
                    users.shuffle(&mut rng);
                    self.santas
                        .update_one(
                            filter.clone(),
                            doc! {"$set": {"game_status": game_status,
                            "users": &users}},
                            None,
                        )
                        .await?;
                    let game = self
                        .santas
                        .find_one(filter.clone(), None)
                        .await?
                        .ok_or(Error::TaskNotFound);
                    return game;
                } else {
                    return Err(Error::IncorrectAdminToken);
                }
            } else if game_status == 1 {
                return Err(Error::GameStarted);
            } else {
                return Err(Error::GameEnded);
            }
        }
    }
    pub async fn end_game(&self, id: &str, admin_token: &str) -> Result<Room, Error> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": obj_id};
        let mut game: Result<Room, Error> = self
            .santas
            .find_one(filter.clone(), None)
            .await?
            .ok_or(Error::TaskNotFound);
        let mut game_status = game.as_mut().unwrap().game_status;
        if game_status.is_none() {
            game_status = Some(0);
        }
        let mut game_status = game_status.unwrap();

        if game_status == 0 {
            return Err(Error::GameNotStarted);
        } else if game_status == 1 {
            let game_token = game.as_ref().unwrap().admin_token.clone();
            if admin_token == game_token {
                game_status = 2;
                self.santas
                    .update_one(
                        filter.clone(),
                        doc! {"$set": {"game_status": game_status}},
                        None,
                    )
                    .await?;
                return game;
            } else {
                return Err(Error::IncorrectAdminToken);
            }
        } else {
            return Err(Error::GameEnded);
        }
    }
}
