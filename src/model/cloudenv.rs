use actix::*;
use actix_web::*;
use apputils::schema::cloudenvironments;
use chrono::NaiveDateTime;
use model::response::{CloudEnviromentsListMsgs};

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Queryable)]
pub struct CloudEnvironments {
    pub id: i32,
    pub env_name: String,
    pub created_at: NaiveDateTime,
}


pub struct CloudEnviromentsList;

impl Message for CloudEnviromentsList {
    type Result = Result<CloudEnviromentsListMsgs, Error>;
}