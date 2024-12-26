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
    get_current_user,
}; 
use crate::views::EmailF;
use actix_session::Session;
use actix_web::dev::ConnectionInfo;
use crate::errors::Error;
use crate::views::AuthResp;


pub fn admin_routes(config: &mut web::ServiceConfig) {
    config.route("/get_users/", web::get().to(get_users));
    config.route("/get_admins/", web::get().to(get_admins));
    config.route("/get_banned_users/", web::get().to(get_banned_users));
    config.route("/get_banned_admins/", web::get().to(get_banned_admins));

    config.route("/block_user/", web::post().to(block_user));
    config.route("/unblock_user/", web::post().to(unblock_user));
    config.route("/block_admin/", web::post().to(block_admin));
    config.route("/unblock_admin/", web::post().to(unblock_admin));
    config.route("/create_admin/", web::post().to(create_admin));
    config.route("/drop_admin/", web::post().to(drop_admin));
    config.route("/create_can_buy/", web::post().to(create_can_buy));
    config.route("/delete_can_buy/", web::post().to(delete_can_buy));
}

#[derive(Deserialize, Serialize)]
pub struct UsersData {
    pub page:  i32,
    pub limit: Option<i64>,
}
#[derive(Deserialize, Serialize)]
pub struct NextPage {
    pub next:  i64,
}

#[derive(Deserialize, Serialize)]
pub struct AuthRespData {
    pub data: Vec<AuthResp>,
    pub next: i64,
}

pub async fn get_users(session: Session, req: HttpRequest) -> Json<AuthRespData> {
    if is_signed_in(&session) {
        let page = crate::utils::get_page(&req);
        let _request_user = get_current_user(&session).expect("E.");
        Json(_request_user.get_users_list(page.into(), Some(20)))
    }
    else {
        Json(AuthRespData {
            data: Vec::new(),
            next: 0,
        })
    }
}
pub async fn get_admins(session: Session, req: HttpRequest) -> Json<AuthRespData> {
    if is_signed_in(&session) {
        let page = crate::utils::get_page(&req);
        let _request_user = get_current_user(&session).expect("E.");
        Json(_request_user.get_admins_list(page.into(), Some(20)))
    }
    else {
        Json(AuthRespData {
            data: Vec::new(),
            next: 0,
        })
    }
}
pub async fn get_banned_users(session: Session, req: HttpRequest) -> Json<AuthRespData> {
    if is_signed_in(&session) {
        let page = crate::utils::get_page(&req);
        let _request_user = get_current_user(&session).expect("E.");
        Json(_request_user.get_banned_users_list(page.into(), Some(20)))
    }
    else {
        Json(AuthRespData {
            data: Vec::new(),
            next: 0,
        })
    }
}
pub async fn get_banned_admins(session: Session, req: HttpRequest) -> Json<AuthRespData> {
    if is_signed_in(&session) {
        let page = crate::utils::get_page(&req);
        let _request_user = get_current_user(&session).expect("E.");
        Json(_request_user.get_banned_admins_list(page.into(), Some(20)))
    }
    else {
        Json(AuthRespData {
            data: Vec::new(),
            next: 0,
        })
    }
}


#[derive(Deserialize, Serialize)]
pub struct ItemId {
    pub id:  i32,
}
pub async fn block_user(session: Session, data: Json<ItemId>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_current_user(&session).expect("E.");
        _request_user.create_user_block(data.id);
    }
    HttpResponse::Ok()
}
pub async fn unblock_user(session: Session, data: Json<ItemId>) -> impl Responder {
        let _request_user = get_current_user(&session).expect("E.");
        if _request_user.perm == 60 {
            _request_user.delete_user_block(data.id);
    }
    HttpResponse::Ok()
}

pub async fn block_admin(session: Session, data: Json<ItemId>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_current_user(&session).expect("E.");
        _request_user.create_admin_block(data.id);
    }
    HttpResponse::Ok()
}
pub async fn unblock_admin(session: Session, data: Json<ItemId>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_current_user(&session).expect("E.");
        _request_user.delete_admin_block(data.id);
    }
    HttpResponse::Ok()
}

pub async fn create_admin(session: Session, data: Json<ItemId>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_current_user(&session).expect("E.");
        _request_user.create_admin(data.id);
    }
    HttpResponse::Ok()
}
pub async fn drop_admin(session: Session, data: Json<ItemId>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_current_user(&session).expect("E.");
        _request_user.delete_admin_block(data.id);
    }
    HttpResponse::Ok()
}

pub async fn create_can_buy(session: Session, data: Json<ItemId>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_current_user(&session).expect("E.");
        _request_user.create_can_buy_token(data.id);
    }
    HttpResponse::Ok()
}
pub async fn delete_can_buy(session: Session, data: Json<ItemId>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_current_user(&session).expect("E.");
        _request_user.delete_can_buy_token(data.id);
    }
    HttpResponse::Ok()
}