use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use std::sync::Arc;

use crate::error::ApiError;
use crate::modules::user::Service;

#[derive(Deserialize)]
pub struct SignupRequest {
    username: String,
    password: String,
}

pub async fn signup(
    service: web::Data<Arc<Service>>,
    request: web::Json<SignupRequest>,
) -> Result<HttpResponse, ApiError> {
    let user = service.create(&request.username, &request.password).await?;
    Ok(HttpResponse::Created().json(user))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn login(
    service: web::Data<Arc<Service>>,
    request: web::Json<LoginRequest>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let user = service
        .authenticate(&request.username, &request.password)
        .await?;

    session
        .insert("user_id", &user.id)
        .expect("To set session id");
    session.renew();

    Ok(HttpResponse::Created().json(user))
}

pub async fn logout(session: Session) -> Result<HttpResponse, ApiError> {
    session.purge();
    Ok(HttpResponse::Ok().finish())
}
