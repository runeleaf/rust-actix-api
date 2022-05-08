use crate::application::*;
use crate::infrastructure::request::message_request::MessageRequest;
use crate::infrastructure::response::*;
use crate::server::middleware::create_context::RequestContext;
use actix_web::{delete, get, post, put, web, web::Json, HttpResponse, Responder};
use log;

#[get("/messages")]
pub async fn index(data: web::Data<RequestContext>) -> impl Responder {
    let application = get_message_service::handle(data.message_repository());
    match application {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(e) => {
            log::error!("{:}", e);
            let response = error_response::ErrorResponse {
                message: format!("{}", e),
                r#type: "messages#index".to_string(),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[get("/messages/{id}")]
pub async fn show(
    data: web::Data<RequestContext>,
    path_params: web::Path<(i32,)>,
) -> impl Responder {
    let message_id = path_params.into_inner().0;
    let application = find_message_service::handle(data.message_repository(), message_id);
    match application {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(e) => {
            log::error!("{:}", e);
            let response = error_response::ErrorResponse {
                message: format!("{}", e),
                r#type: "messages#show".to_string(),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[post("/messages")]
pub async fn create(
    data: web::Data<RequestContext>,
    request: Json<MessageRequest>,
) -> impl Responder {
    log::debug!("{:?}", request);
    let message = MessageRequest::of(&request);
    let application = create_message_service::handle(data.message_repository(), message);
    match application {
        Ok(value) => HttpResponse::Created().json(value),
        Err(e) => {
            log::error!("{:}", e);
            let response = error_response::ErrorResponse {
                message: format!("{}", e),
                r#type: "messages#create".to_string(),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[put("/messages/{id}")]
async fn update(
    data: web::Data<RequestContext>,
    path_params: web::Path<(i32,)>,
    request: Json<MessageRequest>,
) -> impl Responder {
    let message_id = path_params.into_inner().0;
    let mut message = MessageRequest::of(&request);
    message.id = message_id;
    log::debug!("{:?}", request);
    log::debug!("{:?}", message_id);
    let application =
        upsert_message_service::handle(data.message_repository(), message, message_id);
    match application {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(e) => {
            log::error!("{:}", e);
            let response = error_response::ErrorResponse {
                message: format!("{}", e),
                r#type: "messages#create".to_string(),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[delete("/messages/{id}")]
async fn delete(data: web::Data<RequestContext>, path_params: web::Path<(i32,)>) -> impl Responder {
    let message_id = path_params.into_inner().0;
    let application = destroy_message_service::handle(data.message_repository(), message_id);

    match application {
        Ok(_) => HttpResponse::NoContent().json(""),
        Err(e) => {
            log::error!("{:}", e);
            let response = error_response::ErrorResponse {
                message: format!("{}", e),
                r#type: "messages#destroy".to_string(),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}
