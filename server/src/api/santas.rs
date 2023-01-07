use crate::{
    db::mongodb_repo::{Error, MongoRepo},
    models::santa_model::Room
};
use actix_web::{
    error::ResponseError,
    get,
    http::StatusCode,
    post,
    web::{Data, Json, Path},
    Responder,
    delete,
};

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::ObjectIdError(_) | Self::CannotStartAlone => StatusCode::BAD_REQUEST,
            Self::MongoDBError(_) | Self::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::TaskNotFound => StatusCode::NOT_FOUND,
            Self::GameNotStarted | Self::GameStarted | Self::GameEnded => StatusCode::FORBIDDEN,
            Self::IncorrectAdminToken => StatusCode::UNAUTHORIZED
        }
    }
}

#[get("/room/{id}/{name}")]
pub async fn get_room(
    db: Data<MongoRepo>,
    path: Path<(String, String)>,
) -> Result<impl Responder, Error> {
    let (id, name) = path.into_inner();
    db.get_room(id.as_str(), name.as_str()).await.map(Json)
}
#[post("/create_room")]
pub async fn create_room(db: Data<MongoRepo>, room: Json<Room>) -> Result<impl Responder, Error> {
    db.create_room(room.into_inner()).await.map(Json)
}
#[get("/start/{id}/{admin_token}")]
pub async fn start_game(db: Data<MongoRepo>, path: Path<(String, String)>) -> Result<impl Responder, Error> {
    let (id, admin_token) = path.into_inner();
    db.start_game(id.as_str(), admin_token.as_str()).await.map(Json)
}
#[get("/end/{id}/{admin_token}")]
pub async fn end_game(
    db: Data<MongoRepo>,
    path: Path<(String, String)>
) -> Result<impl Responder, Error> {
    let (id, admin_token) = path.into_inner();
    db.end_game(id.as_str(), admin_token.as_str()).await.map(Json)
}
#[delete("/room/{id}/{admin_token}")]
pub async fn delete_room(db: Data<MongoRepo>, info: Path<(String, String)>) -> Result<impl Responder, Error> {
    let (id, admin_token) = info.into_inner();
    db.delete_room(id.as_str(), admin_token.as_str()).await.map(Json)
}
#[delete("/room/{id}/{name}/{admin_token}")]
pub async fn delete_user(db: Data<MongoRepo>, info: Path<(String, String, String)>) -> Result<impl Responder, Error>{
    let (id, name, admin_token) = info.into_inner();
   db.delete_user(id.as_str(), name.as_str(), admin_token.as_str()).await.map(Json)
}
