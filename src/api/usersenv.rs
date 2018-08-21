use actix_web::{HttpMessage, HttpRequest, HttpResponse, State, Json, AsyncResponder, FutureResponse};
use futures::future::Future;

use api::index::AppState;
use model::usersenv::{UsersenvList};

pub fn usersenv(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    unimplemented!();
}

pub fn usersenv_list(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.state().db.send(UsersenvList)
        .from_err()
        .and_then(|res| {
            match res {
                Ok(usersenv_list) => 
                    Ok(HttpResponse::Ok().json(usersenv_list)),
                Err(_) =>
                    Ok(HttpResponse::InternalServerError().into()),
            }
        }).responder()
}