use actix::*;
use actix_web::{Path, fs::NamedFile, HttpRequest, Result};
use std::path::Path as Filepath;
use model::db::ConnDsl;

pub struct AppState {
    pub db: Addr<Syn, ConnDsl>
}

pub fn home(_req: HttpRequest<AppState>) -> Result<NamedFile> {
    Ok(NamedFile::open(Filepath::new("public/index.html"))?)
}

pub fn test(info: Path<(u32, String)>) -> String {
    info!("The index function");
    error!("When an error occurs");
    format!("Hello {} id:{}", info.1, info.0)
}