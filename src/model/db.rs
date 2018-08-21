use actix::*;
use diesel::prelude::PgConnection;
use diesel::r2d2::{ Pool, ConnectionManager };

pub struct ConnDsl(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for ConnDsl {
    type Context = SyncContext<Self>;
}