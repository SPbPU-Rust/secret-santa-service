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
