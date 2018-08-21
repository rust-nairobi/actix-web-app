use actix_web::{HttpMessage, HttpRequest, HttpResponse, State, Json, AsyncResponder, FutureResponse};
use futures::future::Future;

use api::index::AppState;
use model::cloudenv::{CloudEnviromentsList};

pub fn cloudenv(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    unimplemented!();
}

pub fn cloudenv_list(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.state().db.send(CloudEnviromentsList)
        .from_err()
        .and_then(|res| {
            match res {
                Ok(cloudenv_list) => 
                    Ok(HttpResponse::Ok().json(cloudenv_list)),
                Err(_) =>
                    Ok(HttpResponse::InternalServerError().into()),
            }
        }).responder()
}