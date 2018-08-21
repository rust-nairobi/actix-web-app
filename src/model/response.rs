use model::cloudenv::CloudEnvironments;
use model::products::Products;
use model::usersenv::usersenv;

pub enum MyError {
    NotFound,
    DatabaseError,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct CloudEnviromentsListMsgs {
    pub status: i32,
    pub message : String,
    pub cloudenv_list: Vec<CloudEnvironments>,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct ProductsListMsgs {
    pub status: i32,
    pub message : String,
    pub products_list: Vec<Products>,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct UsersenvListMsgs {
    pub status: i32,
    pub message : String,
    pub usersenv_list: Vec<usersenv>,
}