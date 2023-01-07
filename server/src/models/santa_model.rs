use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub admin_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_status: Option<i32> //0 - game created; 1 - game started; 2 - game ended
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusRoom {
    pub my_santa: String,
    pub my_kid: String
}
