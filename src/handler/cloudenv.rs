use diesel;
use diesel::prelude::*;
use actix::*;
use actix_web::*;
use chrono::Utc;
use model::db::ConnDsl;
use model::response::{CloudEnviromentsListMsgs};
use model::cloudenv::{CloudEnviromentsList,CloudEnvironments};

impl Handler<CloudEnviromentsList> for ConnDsl {
    type Result = Result<CloudEnviromentsListMsgs, Error>;

    fn handle(&mut self, cloudenv_list: CloudEnviromentsList, _: &mut Self::Context) -> Self::Result {
        use apputils::schema::cloudenvironments::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let cloudenv = cloudenvironments.load::<CloudEnvironments>(conn).map_err(error::ErrorInternalServerError)?;
        Ok(CloudEnviromentsListMsgs { 
            status: 200,
            message : "cloudenv_list result.".to_string(),
            cloudenv_list: cloudenv,
        })
    }
}