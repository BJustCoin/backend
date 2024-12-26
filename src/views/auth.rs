use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    error::InternalError,
    http::StatusCode,
    dev::ConnectionInfo,
    web::Json,
};
use crate::api_error::ApiError;
use serde::{Deserialize, Serialize};
use crate::utils::{
    is_signed_in,
    verify,
};
use crate::models::{User, SessionUser, EmailVerificationToken, EmailVerificationTokenMessage};
use actix_session::Session;
use crate::errors::AuthError;
use chrono::Utc; 
use uuid::Uuid;


pub fn auth_routes(config: &mut web::ServiceConfig) {
    config.route("/signup/", web::post().to(process_signup));
    config.route("/reset/", web::post().to(process_reset));
    config.route("/login/", web::post().to(login));
    config.route("/invite/", web::post().to(invite));
}


#[derive(Deserialize, Serialize)]
pub struct EmailF {
    pub recipient_name:  String,
    pub recipient_email: String,
    pub subject:         String,
    pub text:            String,
}
#[derive(Deserialize, Serialize)]
struct EmailUserReq {
    name: String,
    email: String,
} 
use reqwest::Client;
use reqwest::header;
use serde_json::json;

#[derive(Debug)]
struct EmailResp {
    status:  String,
}
#[derive(Deserialize, Serialize)]
struct EmailUser {
    name: String,
    email: String,
}

async fn invite(body: web::Json<EmailUserReq>) -> Result<HttpResponse, ApiError> {
    let body = body.into_inner();

    let token_data = EmailVerificationTokenMessage {
        id:  None,
        email: body.email.clone(),
    };
    let token = EmailVerificationToken::create(token_data.clone()).expect("E.");
    let token_string = hex::encode(token.id);
    println!("{}", token_string);
    dotenv::dotenv().ok();
    let api_key = std::env::var("EMAIL_KEY")
        .expect("EMAIL_KEY must be set");
    let sg = sendgrid::SGClient::new(api_key); 
    let mut x_smtpapi = String::new();
    x_smtpapi.push_str(r#"{"unique_args":{"test":7}}"#);

    let text = "Our confirmation code - <strong>".to_owned() + &token_string.to_string() + &"</strong>".to_string();
    let mail_info = sendgrid::Mail::new()
        .add_to(sendgrid::Destination {
            address: &body.email,
            name: &body.name,
        })
        .add_from("no-reply@bjustcoin.com")
        .add_subject("Email confirmation")
        .add_html(&text)
        .add_from_name("BJustcoin Team")
        .add_header("x-cool".to_string(), "indeed")
        .add_x_smtpapi(&x_smtpapi);

    match sg.send(mail_info).await {
        Err(err) => println!("Error: {}", err),
        Ok(body) => println!("Response: {:?}", body),
    };


    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Verification email sent",
    })))
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginUser2 {
    pub email:    String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewUserJson {
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
    pub password:   String,
    pub token:      String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct NewPasswordJson {
    pub email:    String,
    pub password: String,
    pub token:    String,
}

#[derive(Deserialize, Serialize, Debug, Queryable)]
pub struct AuthResp {
    pub id:         i32,
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
    pub perm:       i16,
    pub image:      Option<String>,
    pub phone:      Option<String>,
} 

#[derive(Deserialize, Serialize, Debug, Queryable)]
pub struct AuthResp2 {
    pub id:         i32,
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
    pub perm:       i16,
    pub image:      Option<String>,
    pub phone:      Option<String>,
    pub uuid:       String,
} 

fn find_user(data: Json<LoginUser2>) -> Result<SessionUser, AuthError> {
    let user_some = User::get_user_with_email(&data.email); 
    if user_some.is_ok() { 
        let _user = user_some.expect("Error.");
        if let Ok(matching) = verify(&_user.password, &data.password) {
            if matching {
                let f_user = SessionUser {
                    id:    _user.id,
                    email: _user.email,
                };
                return Ok(f_user.into());
            }
        }
    }
    Err(AuthError::NotFound(String::from("User not found")))
}

fn handle_sign_in (
    data: Json<LoginUser2>,
    req: &HttpRequest
) -> Result<HttpResponse, AuthError> {
    use crate::utils::is_json_request;

    let result = find_user(data);
    let is_json = is_json_request(req);

    match result {
        Ok(user) => {
            if is_json {
                Ok(HttpResponse::Ok().json(user))
            } else {
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
            }
        },
        Err(err) => {
            if is_json {
                Ok(HttpResponse::Unauthorized().json(err.to_string()))
            } else {
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
            }
        },
    }
}


pub async fn login(req: HttpRequest, data: Json<LoginUser2>) -> Json<AuthResp2> {
    if is_signed_in(&req) {
        println!("not anon login");
        return Json(AuthResp2 { 
            id:         0,
            first_name: "".to_string(),
            last_name:  "".to_string(),
            email:      "".to_string(),
            perm:       0,
            image:      None,
            phone:      None,
            uuid:      "".to_string(),
        });
    }
    else {
        let user_some = User::get_user_with_email(&data.email); 
        if user_some.is_ok() {
            println!("user exists");
            let _new_user = user_some.expect("E.");
            handle_sign_in(data, &req);
            return Json(AuthResp2 {
                id:         _new_user.id,
                first_name: _new_user.first_name.clone(),
                last_name:  _new_user.last_name.clone(),
                email:      _new_user.email.clone(),
                perm:       _new_user.perm,
                image:      _new_user.image.clone(),
                phone:      _new_user.phone,
                uuid:       _new_user.uuid,
            });
        }
        else {
            println!("user not found");
            return Json(AuthResp2 {
                id:         0,
                first_name: "".to_string(),
                last_name:  "".to_string(),
                email:      "".to_string(),
                perm:       0,
                image:      None,
                phone:      None,
                uuid:       "".to_string(),
            });
        }
    }
}

pub async fn process_signup(req: HttpRequest, data: Json<NewUserJson>) -> Json<AuthResp2> {
    if is_signed_in(&req) {
        println!("you is not anon!");
        return Json(AuthResp2 {
            id:         0,
            first_name: "".to_string(),
            last_name:  "".to_string(),
            email:      "".to_string(),
            perm:       0,
            image:      None,
            phone:      None,
            uuid:       "".to_string(),
        });
    }
    else { 
        let token_id_res = hex::decode(data.token.clone());
        if token_id_res.is_err() {
            println!("token decode not!");
            return Json(AuthResp2 {
                id:         0,
                first_name: "".to_string(),
                last_name:  "".to_string(),
                email:      "".to_string(),
                perm:       0,
                image:      None,
                phone:      None,
                uuid:       "".to_string(),
            });
        }
        let token_id = token_id_res.expect("E.");
        
        let token_res = EmailVerificationToken::find(&token_id);
        if token_res.is_err() {
            println!("token not found!");
            return Json(AuthResp2 {
                id:         0,
                first_name: "".to_string(),
                last_name:  "".to_string(),
                email:      "".to_string(),
                perm:       0,
                image:      None,
                phone:      None,
                uuid:       "".to_string(),
            });
        }
        let token = token_res.expect("E.");

        if token.expires_at < Utc::now().naive_utc() {
            println!("token expires_at < Utc!");
            return Json(AuthResp2 {
                id:         0,
                first_name: "".to_string(),
                last_name:  "".to_string(),
                email:      "".to_string(),
                perm:       0,
                image:      None,
                phone:      None,
                uuid:       "".to_string(),
            });
        }

        let _new_user = User::create(data);

        let _session_user = SessionUser {
            id:    _new_user.id,
            email: _new_user.email.clone(),
        };

        println!("Yes!!");

        dotenv::dotenv().ok();
        let api_key = std::env::var("EMAIL_KEY")
            .expect("EMAIL_KEY must be set");
        let sg = sendgrid::SGClient::new(api_key); 
        let mut x_smtpapi = String::new();
        x_smtpapi.push_str(r#"{"unique_args":{"test":7}}"#);

        let text = "A new user - <strong>".to_owned() 
            + &_new_user.first_name.clone() 
            + &" ".to_string() 
            + &_new_user.last_name.clone() 
            + &"</strong> has signed up for BJustcoin. Link to the list of users - ".to_string()
            + &"https://dashboard.bjustcoin.com/users/".to_string();
        let mail_info = sendgrid::Mail::new()
            .add_to(sendgrid::Destination {
                address: "interesnijsim49293@gmail.com",
                name: "Sergei Zubarev",
            })
            .add_from("no-reply@bjustcoin.com")
            .add_subject("New user in BJustCoin")
            .add_html(&text)
            .add_from_name("BJustcoin Team")
            .add_header("x-cool".to_string(), "indeed")
            .add_x_smtpapi(&x_smtpapi);

        match sg.send(mail_info).await {
            Err(err) => println!("Error: {}", err),
            Ok(body) => println!("Response: {:?}", body),
        };
        println!("mail send!");

        return Json(AuthResp2 {
            id:         _new_user.id,
            first_name: _new_user.first_name.clone(),
            last_name:  _new_user.last_name.clone(),
            email:      _new_user.email.clone(),
            perm:       _new_user.perm,
            image:      _new_user.image,
            phone:      None,
            uuid:       _new_user.uuid,
        })
    }
}

pub async fn process_reset(req: HttpRequest, data: Json<NewPasswordJson>) -> Json<AuthResp> {
    if is_signed_in(&req) {
        return Json(AuthResp {
            id:         0,
            first_name: "".to_string(),
            last_name:  "".to_string(),
            email:      "".to_string(),
            perm:       0,
            image:      None,
            phone:      None,
        }); 
    }
    else { 
        let token_id_res = hex::decode(data.token.clone());
        if token_id_res.is_err() {
            return Json(AuthResp {
                id:         0,
                first_name: "".to_string(),
                last_name:  "".to_string(),
                email:      "".to_string(),
                perm:       0,
                image:      None,
                phone:      None,
            });
        }
        let token_id = token_id_res.expect("E.");
        
        let token_res = EmailVerificationToken::find(&token_id);
        if token_res.is_err() {
            return Json(AuthResp {
                id:         0,
                first_name: "".to_string(),
                last_name:  "".to_string(),
                email:      "".to_string(),
                perm:       0,
                image:      None,
                phone:      None,
            });
        }
        let token = token_res.expect("E.");

        if token.email != data.email {
            return Json(AuthResp {
                id:         0,
                first_name: "".to_string(),
                last_name:  "".to_string(),
                email:      "".to_string(),
                perm:       0,
                image:      None,
                phone:      None,
            });
        }

        if token.expires_at < Utc::now().naive_utc() {
            return Json(AuthResp {
                id:         0,
                first_name: "".to_string(),
                last_name:  "".to_string(),
                email:      "".to_string(),
                perm:       0,
                image:      None,
                phone:      None,
            });
        }

        let _user_res = User::get_user_with_email(&data.email);
        if _user_res.is_ok() {
            let _user = _user_res.expect("E.");
            let _session_user = SessionUser {
                id:    _user.id,
                email: _user.email.clone(),
            };

            return Json(AuthResp {
                id:         _user.id,
                first_name: _user.first_name.clone(),
                last_name:  _user.last_name.clone(),
                email:      _user.email.clone(),
                perm:       _user.perm,
                image:      _user.image.clone(),
                phone:      _user.phone,
            })
        }
        else {
            return Json(AuthResp {
                id:         0,
                first_name: "".to_string(),
                last_name:  "".to_string(),
                email:      "".to_string(),
                perm:       0,
                image:      None,
                phone:      None,
            });
        }
    }
}