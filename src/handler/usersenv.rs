use diesel;
use diesel::prelude::*;
use actix::*;
use actix_web::*;
use chrono::Utc;
use model::db::ConnDsl;
use model::response::{UsersenvListMsgs};
use model::usersenv::{UsersenvList,usersenv};

impl Handler<UsersenvList> for ConnDsl {
    type Result = Result<UsersenvListMsgs, Error>;

    fn handle(&mut self, usersenv_list: UsersenvList, _: &mut Self::Context) -> Self::Result {
        use apputils::schema::usersenvs::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let envs = usersenvs.load::<usersenv>(conn).map_err(error::ErrorInternalServerError)?;
        Ok(UsersenvListMsgs { 
            status: 200,
            message : "usersenv_list result.".to_string(),
            usersenv_list: envs,
        })
    }
}