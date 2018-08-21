use actix::*;
use actix_web::*;
use apputils::schema::usersenvs;
use chrono::{DateTime, NaiveDateTime};
use model::response::{UsersenvListMsgs};

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Queryable)]
pub struct usersenv {
    pub id: i32,
    pub user_id: i32,
    pub env_id: i32,
    pub po_id: i32,
    pub start_date: NaiveDateTime,
    pub max_duration: String,
    pub lease_period: String,
    pub status: i32,
    pub env_type: String,
    pub created_at: NaiveDateTime,
}
 
pub struct UsersenvList;

impl Message for UsersenvList {
    type Result = Result<UsersenvListMsgs, Error>;
}
