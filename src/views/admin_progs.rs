use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    web::Json,
};
use crate::models::{
    User,
};
use serde::{Deserialize, Serialize};

use crate::utils::{
    is_signed_in,
    get_request_user,
    send_email,
    EmailF,
}; 
use actix_session::Session;
use actix_web::dev::ConnectionInfo;
use crate::errors::Error;
use crate::views::AuthResp;


pub fn admin_routes(config: &mut web::ServiceConfig) {
    config.route("/get_users/", web::get().to(get_users));
    config.route("/block_user/", web::post().to(block_user));
}

#[derive(Deserialize, Serialize)]
pub struct UsersData {
    pub page:  i32,
    pub limit: Option<i64>,
}
pub async fn get_users(session: Session, data: Json<UsersData>) -> (Json<Vec<AuthResp>, i32) {
    if is_signed_in(&session) {
        let _request_user = get_request_user(&session);
        _request_user.get_users_list(data.page, data.limit)
    }
    else {
        Json(AuthResp { 
            id:         0,
            first_name: "".to_string(),
            last_name:  "".to_string(),
            email:      "".to_string(),
            perm:       0,
            image:      None,
            phone:      None,
        }, 0)
    }
    HttpResponse::Ok()
}

#[derive(Deserialize, Serialize)]
pub struct ItemId {
    pub id:  i32,
}
pub async fn block_user(session: Session, data: Json<ItemId>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user(&session);
        _request_user.create_user_block(data.id);
    }
    HttpResponse::Ok()
}
