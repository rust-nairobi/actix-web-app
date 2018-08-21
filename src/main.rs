#![allow(warnings)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
use std::io::Write;
extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate num_cpus;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate dotenv;
extern crate chrono;
extern crate bcrypt;
extern crate http;
extern crate ring;
extern crate data_encoding;
extern crate postgres;

use actix::*;
use actix_web::{server, App, http::{header, Method}, fs, 
middleware, middleware::cors::Cors};
use diesel::prelude::PgConnection;
use diesel::r2d2::{ Pool, ConnectionManager };

mod api;
mod model;
mod apputils;
mod handler;

use model::db::ConnDsl;
use api::index::{AppState, home, test};
use api::cloudenv::{cloudenv_list};
use api::usersenv::{usersenv_list};
use api::products::{productowner_list, products_list};

use env_logger::{Builder, Env};

fn init_logger() {
    let env = Env::default()
        .filter("MY_LOG_LEVEL")
        .write_style("MY_LOG_STYLE");

    let mut builder = Builder::from_env(env);

    builder.format(|buf, record| {
        let timestamp = buf.timestamp();
        writeln!(buf, "{} [{}] - {}",
                timestamp,
                record.level(),
                record.args())
    });

    builder.init();
}

fn main() {
    init_logger();

    let sys = System::new("Cloud-server");

    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let conn = Pool::builder().build(manager).expect("Failed to create pool.");
    let addr = SyncArbiter::start( num_cpus::get() * 4, move || { ConnDsl(conn.clone()) });

    server::new(move || {
        App::with_state(AppState { db: addr.clone() })
        .middleware(middleware::Logger::default())
        .resource("/", |r| r.h(home)) 
        .route("/{id}/{name}/", http::Method::GET, test)
        .configure(|app| Cors::for_app(app)
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .max_age(3600)
        .resource("/api/env_list", |r| { r.method(Method::POST).with(cloudenv_list); })
        .resource("/api/usersenv_list", |r| { r.method(Method::POST).with(usersenv_list); })
        .resource("/api/po_list", |r| { r.method(Method::POST).with(productowner_list); })
        .resource("/api/product_list", |r| { r.method(Method::POST).with(products_list); })
        .register())
        })
        .bind("127.0.0.1:8083").unwrap()
        .shutdown_timeout(2)
        .start();

    info!("Cloud server started ... ");

    let _ = sys.run();

}
