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
use crate::schema;
use crate::diesel::{
    Queryable,
    Insertable,
    QueryDsl,
    ExpressionMethods,
    RunQueryDsl,
    Connection,
};
use crate::utils::establish_connection;


pub fn admin_routes(config: &mut web::ServiceConfig) {
    config.route("/get_holders/", web::get().to(get_holders));
    config.route("/get_small_users/", web::get().to(get_small_users));
    config.route("/get_users/", web::get().to(get_users)); 
    config.route("/get_admins/", web::get().to(get_admins));
    config.route("/get_banned_users/", web::get().to(get_banned_users));
    config.route("/get_banned_admins/", web::get().to(get_banned_admins)); 

    config.route("/get_logs/", web::get().to(get_logs));
    config.route("/get_user_logs/", web::get().to(get_user_logs)); 

    config.route("/get_new_applications/", web::get().to(get_new_applications));
    config.route("/get_approved_applications/", web::get().to(get_approved_applications));
    config.route("/get_rejected_applications/", web::get().to(get_rejected_applications));

    config.route("/block_user/", web::post().to(block_user));
    config.route("/unblock_user/", web::post().to(unblock_user));
    config.route("/block_admin/", web::post().to(block_admin));
    config.route("/unblock_admin/", web::post().to(unblock_admin));
    config.route("/create_admin/", web::post().to(create_admin));
    config.route("/drop_admin/", web::post().to(drop_admin));
    config.route("/agree_application/", web::post().to(agree_application));
    config.route("/create_suggest_item/", web::post().to(create_suggest_item));
    config.route("/create_log/", web::post().to(create_log));
    config.route("/create_holders/", web::post().to(create_holders));
    config.route("/delete_holder/", web::post().to(delete_holder));
    config.route("/send_mail/", web::post().to(send_mail));
    config.route("/subscribe/", web::post().to(subscribe));
}

pub async fn get_holders(req: HttpRequest) -> Json<crate::models::HolderRespData> {
        let page = crate::utils::get_page(&req);
        Json(crate::models::Holder::get_list(page.into(), Some(20)))
}
pub async fn get_new_applications(req: HttpRequest) -> Json<crate::models::SuggestRespData> {
    if is_signed_in(&req) {
        let page = crate::utils::get_page(&req);
        let _request_user = get_current_user(&req);
        Json(crate::models::SuggestItem::get_new_list(page.into(), Some(20)))
    }
    else {
        Json(crate::models::SuggestRespData {
            data: Vec::new(),
            next: 0,
        })
    }
}
pub async fn get_approved_applications(req: HttpRequest) -> Json<crate::models::SuggestRespData> {
    if is_signed_in(&req) {
        let page = crate::utils::get_page(&req);
        let _request_user = get_current_user(&req);
        Json(crate::models::SuggestItem::get_approved_list(page.into(), Some(20)))
    }
    else {
        Json(crate::models::SuggestRespData {
            data: Vec::new(),
            next: 0,
        })
    }
}
pub async fn get_rejected_applications(req: HttpRequest) -> Json<crate::models::SuggestRespData> {
    if is_signed_in(&req) {
        let page = crate::utils::get_page(&req);
        let _request_user = get_current_user(&req);
        Json(crate::models::SuggestItem::get_rejected_list(page.into(), Some(20)))
    }
    else {
        Json(crate::models::SuggestRespData {
            data: Vec::new(),
            next: 0,
        })
    }
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
        //if _request_user.email.clone() == "".to_string() {
//
        //}
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ReqWallet {
    pub id:        i32,
    pub tokens:    String,
    pub ico_stage: i16,
}
pub async fn agree_application(req: HttpRequest, data: Json<ReqWallet>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.is_superuser() {
            crate::models::SuggestItem::agree_application(data.id, data.tokens.clone(), data.ico_stage);
            let user_data = crate::models::SuggestItem::get_user_data(data.id);
            
            dotenv::dotenv().ok(); 
            let api_key = std::env::var("EMAIL_KEY")
                .expect("EMAIL_KEY must be set");
            let sg = sendgrid::SGClient::new(api_key); 
            let mut x_smtpapi = String::new();
            x_smtpapi.push_str(r#"{"unique_args":{"test":7}}"#);

            // mail for Beatrice
            let text = "<div><strong>Dear ".to_string() + &user_data.first_name + &"</strong><br /><br />Congratulations! Your purchase of <strong>BJustCoin (BJC)</strong> has been <strong>approved</strong>. You’re just one step away from securing your requested amount.<br /><br />To complete your transaction, please follow the link below:<br /><br /><a href='https://dashboard.bjustcoin.com/profile/' target='_blank'>🔗 Complete Your Purchase Now</a><br /><br />Once your purchase is finalized, we’ll handle the rest and ensure your BJC is securely delivered to your wallet.<br /><br />If you have any questions or need assistance, feel free to reach out to our support team at <strong>Corporate@bjustcoin.com</strong><br /><br />Thank you for choosing <strong>BJustCoin</strong>—welcome to the future of digital transactions!<br /><br />Best regards,<br /><strong>The BJustCoin Team</strong></div>".to_string();
            let name = user_data.first_name.clone() + &" ".to_string() + &user_data.last_name;
            let mail_info = sendgrid::Mail::new() 
                .add_to(sendgrid::Destination {
                    address: &user_data.email,
                    name: &name,
                })
                .add_from("no-reply@bjustcoin.com")
                .add_subject("Your BJustCoin Purchase is Approved – Complete Your Transaction Now!")
                .add_html(&text)
                .add_from_name("BJustcoin Team")
                .add_header("x-cool".to_string(), "indeed")
                .add_x_smtpapi(&x_smtpapi);

            match sg.send(mail_info).await {
                Err(err) => println!("Error: {}", err),
                Ok(body) => println!("Response: {:?}", body),
            };
            println!("mail send!");
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
        let text = "Your application for token purchase was submitted! Thank you for your interest! We’re thrilled by the incredible response and appreciate your enthusiasm. Due to the high volume of demand, we are currently experiencing a slight delay in processing orders. Rest assured, our team is working tirelessly to get your purchases to you as quickly as possible. Thank you for your patience and support—it means the world to us. Stay tuned for updates, and we can’t wait for you to complete your purchase of BJustCoin.".to_string();
        let first_name = _request_user.first_name.clone();
        let last_name = _request_user.last_name.clone();
        let email = _request_user.email.clone();
        let name = first_name + &" ".to_string() + &last_name;
        let mail_info = sendgrid::Mail::new() 
            .add_to(sendgrid::Destination {
                address: &email,
                name: &name,
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
        crate::models::SuggestItem::create (
            data, 
            _request_user.first_name.clone(),
            _request_user.last_name.clone(),
            _request_user.email.clone()
        );
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
pub async fn create_holders(req: HttpRequest, data: Json<Vec<crate::models::NewHolder>>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        crate::models::Holder::create(data);
    }
    HttpResponse::Ok()
}
#[derive(Deserialize, Serialize, Debug)]
pub struct DataId { 
    pub id: i32,
}  
pub async fn delete_holder(req: HttpRequest, data: Json<DataId>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        crate::models::Holder::delete(data.id);
    }
    HttpResponse::Ok()
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SendMailJson { 
    pub subtitle:   String,
    pub text:       String,
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
}   
pub async fn send_mail(req: HttpRequest, data: Json<SendMailJson>) -> impl Responder {
    if is_signed_in(&req) {
        let _connection = establish_connection();
        let _request_user = get_current_user(&req);

        let first_name: String;
        let last_name: String;
        let email: String;
    
        dotenv::dotenv().ok();
        let api_key = std::env::var("EMAIL_KEY")
            .expect("EMAIL_KEY must be set");
        let sg = sendgrid::SGClient::new(api_key); 
        let mut x_smtpapi = String::new();
        x_smtpapi.push_str(r#"{"unique_args":{"test":7}}"#);

        let name = data.first_name.clone() + &" ".to_string() + &data.last_name;
        let mail_info = sendgrid::Mail::new() 
            .add_to(sendgrid::Destination {
                address: &data.email,
                name:    &name,
            })
            .add_from("no-reply@bjustcoin.com")
            .add_subject(&data.subtitle)
            .add_html(&data.text)
            .add_from_name("BJustcoin Team")
            .add_header("x-cool".to_string(), "indeed")
            .add_x_smtpapi(&x_smtpapi);

        match sg.send(mail_info).await {
            Err(err) => println!("Error: {}", err),
            Ok(body) => println!("Response: {:?}", body),
        };
        println!("mail send!");
    }
    HttpResponse::Ok()
}


#[derive(Deserialize, Serialize, Debug)]
pub struct SendSubscribeMailJson {
    pub email: String,
} 
pub async fn subscribe(req: HttpRequest, data: Json<SendSubscribeMailJson>) -> impl Responder {
        dotenv::dotenv().ok();
        let api_key = std::env::var("EMAIL_KEY")
            .expect("EMAIL_KEY must be set");
        let sg = sendgrid::SGClient::new(api_key); 
        let mut x_smtpapi = String::new();
        x_smtpapi.push_str(r#"{"unique_args":{"test":7}}"#);

        // mail for subscriber
        let mail_info = sendgrid::Mail::new() 
            .add_to(sendgrid::Destination {
                address: &data.email,
                name:    "BJustCoin Community Member",
            })
            .add_from("no-reply@bjustcoin.com")
            .add_subject("Join Us in the Seed Round of BJustCoin ICO!")
            .add_html("Dear BJustCoin Community Member, We’re thrilled to have you as part of our exciting journey! We are currently in the Seed Round of our ICO, and this is your opportunity to be an integral part of the growth and future of BJustCoin. Feel free to join us and make your purchase via the official ICO link: https://etherscan.io/address/0xf86082F6bf8BD9FFC02755f65FC3d7eC7d1A0ffc. Your support and belief in our vision mean the world to us, and we’re excited to build the future together with you. Let’s make it happen! Warm regards, The BJustCoin Team")
            .add_from_name("BJustcoin Team")
            .add_header("x-cool".to_string(), "indeed")
            .add_x_smtpapi(&x_smtpapi);

        match sg.send(mail_info).await {
            Err(err) => println!("Error: {}", err),
            Ok(body) => println!("Response: {:?}", body),
        };

        // mail for Beatrice
        let mail_info = sendgrid::Mail::new() 
            .add_to(sendgrid::Destination {
                address: "beatrice.obrien@justlaw.com",
                name:    "Beatrice O'Brien",
            })
            .add_from("no-reply@bjustcoin.com")
            .add_subject("ICO Email Monitoring Notification")
            .add_html("Dear Beatrice. You are receiving this notification as part of your role in monitoring ICO-related communications for BJustCoin.")
            .add_from_name("BJustcoin Team")
            .add_header("x-cool".to_string(), "indeed")
            .add_x_smtpapi(&x_smtpapi);

        match sg.send(mail_info).await {
            Err(err) => println!("Error: {}", err),
            Ok(body) => println!("Response: {:?}", body),
        };

        // mail for Colin
        let mail_info = sendgrid::Mail::new() 
            .add_to(sendgrid::Destination {
                address: "colin@bjustcoin.com",
                name:    "Colin Martin",
            })
            .add_from("no-reply@bjustcoin.com")
            .add_subject("ICO Email Monitoring Notification")
            .add_html("Dear Colin. You are receiving this notification as part of your role in monitoring ICO-related communications for BJustCoin.")
            .add_from_name("BJustcoin Team")
            .add_header("x-cool".to_string(), "indeed")
            .add_x_smtpapi(&x_smtpapi);

        match sg.send(mail_info).await {
            Err(err) => println!("Error: {}", err),
            Ok(body) => println!("Response: {:?}", body),
        };
    HttpResponse::Ok()
}