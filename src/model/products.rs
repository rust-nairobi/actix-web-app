use actix::*;
use actix_web::*;
use apputils::schema::products;
use chrono::NaiveDateTime;
use model::response::{ProductsListMsgs, ProductOwnerListMsgs};


#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Queryable)]
pub struct Products {
    pub id: i32,
    pub prod_name: String,
    pub created_at: NaiveDateTime,
}

pub struct ProductsList;

impl Message for ProductsList {
    type Result = Result<ProductsListMsgs, Error>;
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Queryable)]
pub struct ProductOwner {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
}

pub struct ProductOwnerList;

impl Message for ProductOwnerList {
    type Result = Result<ProductOwnerListMsgs, Error>;
}
