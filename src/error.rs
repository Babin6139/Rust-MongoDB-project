use serde::Serialize;
use thiserror::Error;
use warp::{http::StatusCode,reply,Rejection,Reply};
use std::convert::Infallible;
use mongodb::bson;


#[derive(Debug,Error)]
pub enum Error{
    #[error("mongodb error:{0}")]
    MongoError(#[from]mongodb::error::Error),
    #[error("error during mongodb query:{0}")]
    MongoQueryError(mongodb::error::Error),
    #[error("could not access field in document: {0}")]
    MongoDataError(#[from]bson::document::ValueAccessError),
    #[error("invalid id used : {0}")]
    InvalidIDError(String),
}

#[derive(Serialize)]
struct  ErrorResponse{
    message:String,
}

impl warp::reject::Reject for Error{
}
pub async fn handle_rejection(err:Rejection)->std::result::Result<Box<dyn Reply>,Infallible>{
    let mut code:StatusCode;
    let mut message:&str;

    if err.is_not_found(){
        code=StatusCode::NOT_FOUND;
        message="Not Found";
    }else if let Some(_)=err.find::<warp::filters::body::BodyDeserializeError>(){
        code=StatusCode::BAD_REQUEST;
        message="Invalid Body";
    }else if let Some(e)=err.find::<Error>()  {
        match e{
            _=>{
                eprintln!("unhandled application error: {:?}",err);
                code=StatusCode::INTERNAL_SERVER_ERROR;
                message="Internal Server Error";
            }
        }
    }
}

