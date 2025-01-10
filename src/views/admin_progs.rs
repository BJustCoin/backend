use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    web::Json,
};
use crate::models::{
    User, SmallUsers,
};
use serde::{Deserialize, Serialize};

use crate::utils::{
    is_signed_in,
    get_current_user,
}; 
use crate::views::AuthResp;


pub fn admin_routes(config: &mut web::ServiceConfig) {
    config.route("/get_small_users/", web::get().to(get_small_users));
    config.route("/get_users/", web::get().to(get_users)); 
    config.route("/get_admins/", web::get().to(get_admins));
    config.route("/get_banned_users/", web::get().to(get_banned_users));
    config.route("/get_banned_admins/", web::get().to(get_banned_admins)); 

    config.route("/get_logs/", web::get().to(get_logs));
    config.route("/get_user_logs/", web::get().to(get_user_logs));
    config.route("/get_suggest_items/", web::get().to(get_suggest_items));

    config.route("/block_user/", web::post().to(block_user));
    config.route("/unblock_user/", web::post().to(unblock_user));
    config.route("/block_admin/", web::post().to(block_admin));
    config.route("/unblock_admin/", web::post().to(unblock_admin));
    config.route("/create_admin/", web::post().to(create_admin));
    config.route("/drop_admin/", web::post().to(drop_admin));
    config.route("/create_can_buy/", web::post().to(create_can_buy));
    config.route("/delete_can_buy/", web::post().to(delete_can_buy));
    config.route("/create_wallet/", web::post().to(create_wallet));
    config.route("/delete_wallet/", web::post().to(delete_wallet));
    config.route("/create_white_list/", web::post().to(create_white_list));
    config.route("/delete_white_list/", web::post().to(delete_white_list));
    config.route("/create_suggest_item/", web::post().to(create_suggest_item));
    config.route("/create_log/", web::post().to(create_log));
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

pub async fn get_users(req: HttpRequest) -> Json<AuthRespData> {
    if is_signed_in(&req) {
        let page = crate::utils::get_page(&req);
        let _request_user = get_current_user(&req);
        Json(User::get_users_list(page.into(), Some(20)))
    }
    else {
        Json(AuthRespData {
            data: Vec::new(),
            next: 10,
        })
    }
}
pub async fn get_small_users(req: HttpRequest) -> Json<SmallUsers> {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        Json(SmallUsers {
            users: User::get_small_users(),
        })
    }
    else {
        Json(SmallUsers {
            users: Vec::new(),
        })
    }
}

pub async fn get_admins(req: HttpRequest) -> Json<AuthRespData> {
    if is_signed_in(&req) {
        let page = crate::utils::get_page(&req);
        let _request_user = get_current_user(&req);
        Json(User::get_admins_list(page.into(), Some(20)))
    }
    else {
        Json(AuthRespData {
            data: Vec::new(),
            next: 0,
        })
    }
}
pub async fn get_banned_users(req: HttpRequest) -> Json<AuthRespData> {
    if is_signed_in(&req) {
        let page = crate::utils::get_page(&req);
        let _request_user = get_current_user(&req);
        if _request_user.perm == 60 {
            Json(User::get_banned_users_list(page.into(), Some(20)))
        }
        else {
            Json(AuthRespData {
                data: Vec::new(),
                next: 0,
            })
        }
    }
    else {
        Json(AuthRespData {
            data: Vec::new(),
            next: 0,
        })
    }
}
pub async fn get_banned_admins(req: HttpRequest) -> Json<AuthRespData> {
    if is_signed_in(&req) {
        let page = crate::utils::get_page(&req);
        let _request_user = get_current_user(&req);
        Json(User::get_banned_admins_list(page.into(), Some(20)))
    }
    else {
        Json(AuthRespData {
            data: Vec::new(),
            next: 0,
        })
    }
}

pub async fn get_logs(req: HttpRequest) -> Json<crate::models::LogRespData> {
    if is_signed_in(&req) {
        let page = crate::utils::get_page(&req);
        let _request_user = get_current_user(&req);
        Json(crate::models::Log::get_list(page.into(), Some(20)))
    }
    else {
        Json(crate::models::LogRespData {
            data: Vec::new(),
            next: 0,
        })
    }
}
pub async fn get_user_logs(req: HttpRequest) -> Json<crate::models::LogRespData> {
    if is_signed_in(&req) {
        let page = crate::utils::get_page(&req);
        let id = crate::utils::get_id(&req);
        let _request_user = get_current_user(&req);
        Json(crate::models::Log::get_list_for_user(id, page.into(), Some(20)))
    }
    else {
        Json(crate::models::LogRespData {
            data: Vec::new(),
            next: 0,
        })
    }
}
pub async fn get_suggest_items(req: HttpRequest) -> Json<crate::models::SuggestRespData> {
    if is_signed_in(&req) {
        let page = crate::utils::get_page(&req);
        let _request_user = get_current_user(&req);
        Json(crate::models::SuggestItem::get_list(page.into(), Some(20)))
    }
    else {
        Json(crate::models::SuggestRespData {
            data: Vec::new(),
            next: 0,
        })
    }
}


#[derive(Deserialize, Serialize)]
pub struct ItemId {
    pub id:  i32,
}
pub async fn block_user(req: HttpRequest, data: Json<ItemId>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        _request_user.create_user_block(data.id);
    }
    HttpResponse::Ok()
}
pub async fn unblock_user(req: HttpRequest, data: Json<ItemId>) -> impl Responder {
        let _request_user = get_current_user(&req);
        if _request_user.perm == 60 {
            _request_user.delete_user_block(data.id);
    }
    HttpResponse::Ok()
}

pub async fn block_admin(req: HttpRequest, data: Json<ItemId>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        _request_user.create_admin_block(data.id);
    }
    HttpResponse::Ok()
}
pub async fn unblock_admin(req: HttpRequest, data: Json<ItemId>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        _request_user.delete_admin_block(data.id);
    }
    HttpResponse::Ok()
}

pub async fn create_admin(req: HttpRequest, data: Json<ItemId>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        _request_user.create_admin(data.id);
    }
    HttpResponse::Ok()
}
pub async fn drop_admin(req: HttpRequest, data: Json<ItemId>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        _request_user.delete_admin_block(data.id);
    }
    HttpResponse::Ok()
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ItemIdTypes {
    pub id:    i32,
    pub types: i16,
}
pub async fn create_can_buy(req: HttpRequest, data: Json<ItemIdTypes>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.is_superuser() {
            _request_user.create_can_buy_token(data.id, data.types);
        }
    } 
    HttpResponse::Ok()
}
pub async fn delete_can_buy(req: HttpRequest, data: Json<ItemIdTypes>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.is_superuser() {
            _request_user.delete_can_buy_token(data.id, data.types);
        }
    }
    HttpResponse::Ok()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReqWallet {
    pub user_id:   i32,
    pub link:      String,
    pub ico_stage: i16,
}
pub async fn create_wallet(req: HttpRequest, data: Json<ReqWallet>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.is_superuser() {
            crate::models::NewWallet::create(data.user_id, data.link.clone(), data.ico_stage);
        }
    }
    HttpResponse::Ok()
}
pub async fn delete_wallet(req: HttpRequest, data: Json<ItemId>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.is_superuser() {
            crate::models::NewWallet::delete(data.id);
        }
    }
    HttpResponse::Ok()
}

pub async fn create_white_list(req: HttpRequest, data: Json<crate::models::NewNewWhiteList>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.is_superuser() {
            crate::models::NewWhiteList::create(data.user_id, data.token_type);
        }
    }
    HttpResponse::Ok()
}
pub async fn delete_white_list(req: HttpRequest, data: Json<ItemId>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.is_superuser() {
            crate::models::NewWallet::delete(data.id);
        } 
    }
    HttpResponse::Ok()
}
pub async fn create_suggest_item(req: HttpRequest, data: Json<crate::models::NewSuggestJson>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
    
        dotenv::dotenv().ok();
        let api_key = std::env::var("EMAIL_KEY")
            .expect("EMAIL_KEY must be set");
        let sg = sendgrid::SGClient::new(api_key); 
        let mut x_smtpapi = String::new();
        x_smtpapi.push_str(r#"{"unique_args":{"test":7}}"#);

        // mail for Beatrice
        let text = "A new BJustCoin purchase interest has triggered. Link to the list - ".to_string()
            + &"https://dashboard.bjustcoin.com/suggest_items/".to_string();
        let mail_info = sendgrid::Mail::new() 
            .add_to(sendgrid::Destination {
                address: "Beatrice.OBrien@justlaw.com",
                name: "Beatrice OBrien",
            })
            .add_from("no-reply@bjustcoin.com")
            .add_subject("New Application for Token purchase")
            .add_html(&text)
            .add_from_name("BJustcoin Team")
            .add_header("x-cool".to_string(), "indeed")
            .add_x_smtpapi(&x_smtpapi);

        match sg.send(mail_info).await {
            Err(err) => println!("Error: {}", err),
            Ok(body) => println!("Response: {:?}", body),
        };
        println!("mail send!");

        // mail for request user
        let text = "Your application for token purchase was submitted! Thank you.".to_string();
        let first_name = data.first_name.clone();
        let last_name = data.last_name.clone();
        let mail_info = sendgrid::Mail::new() 
            .add_to(sendgrid::Destination {
                address: &data.email.clone(),
                name: &(first_name + &" ".to_string() + &last_name),
            })
            .add_from("no-reply@bjustcoin.com")
            .add_subject("Your application was submitted!")
            .add_html(&text)
            .add_from_name("BJustcoin Team")
            .add_header("x-cool".to_string(), "indeed")
            .add_x_smtpapi(&x_smtpapi);

        match sg.send(mail_info).await {
            Err(err) => println!("Error: {}", err),
            Ok(body) => println!("Response: {:?}", body),
        };
        println!("mail send!");
        crate::models::SuggestItem::create(data);
    }
    HttpResponse::Ok()
}
pub async fn create_log(req: HttpRequest, data: Json<crate::models::NewLogJson>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        crate::models::Log::create(data);
    }
    HttpResponse::Ok()
}