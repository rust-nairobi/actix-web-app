use actix::*;
use actix_web::*;
use apputils::schema::products;
use chrono::NaiveDateTime;
use model::response::ProductsListMsgs;


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