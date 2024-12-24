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
    config.route("/logout/", web::get().to(logout));
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

#[derive(Deserialize, Serialize, Debug)]
struct Personalization {
    personalizations:  PersonalizationsData,
    from:              FromData,
    subject:           String,
    content:           ContentData,
}
#[derive(Deserialize, Serialize, Debug)]
struct EmailNameData {
    email:  String,
    name:  String,
}
#[derive(Deserialize, Serialize, Debug)]
struct TypeValueData {
    r#type: String,
    value:  String,
}

#[derive(Deserialize, Serialize, Debug)]
struct PersonalizationsData {
    from: EmailNameData,
    to:   EmailNameData,
}
#[derive(Deserialize, Serialize, Debug)]
struct FromData {
    from: EmailNameData,
}
#[derive(Deserialize, Serialize, Debug)]
struct ContentData {
    r#type: String,
    value:   String,
}

pub async fn send_email(data: EmailF) -> bool {
    dotenv::dotenv().ok();
    let api_key = std::env::var("EMAIL_KEY")
        .expect("EMAIL_KEY must be set");
    let sender = EmailUser {
        name: "BJustCoin Team".to_string(),
        email: "no-reply@bjustcoin.com".to_string(),
    }; 

    let recipient = EmailUser {
        name: data.recipient_name.clone(),
        email: data.recipient_email.clone(),
    };

    let body = json!({
            "personalizations": [{
                "from": {
                    "email": sender.email.clone(),
                    "name": sender.name.clone()
                },
                "to": [{
                    "email": recipient.email.clone(),
                    "name": recipient.name.clone()
                }]
            }],
            "from": {
                "email": sender.email.clone(),
                "name": sender.name.clone()
            },
            "subject": data.subject.clone(),
            "content": [
                {
                    "type": "text/plain",
                    "value": data.text.clone()
                },
            ]
        });
    let client = Client::new()
        .post("https://api.sendgrid.com/v3/mail/send")
        .json(&body)
        .bearer_auth(api_key)
        .header(
            header::CONTENT_TYPE, 
            header::HeaderValue::from_static("application/json")
        );

    let response = client.send();
    if response.await.is_ok() {
        println!("200");
        return true
    }
    else {
        println!("400");
        return false
    }
}


async fn invite(body: web::Json<EmailUserReq>) -> Result<HttpResponse, ApiError> {
    let body = body.into_inner();

    let token_data = EmailVerificationTokenMessage {
        id:  None,
        email: body.email.clone(),
    };
    let token = EmailVerificationToken::create(token_data.clone()).expect("E.");
    let token_string = hex::encode(token.id);

    dotenv::dotenv().ok();
    let api_key = std::env::var("EMAIL_KEY")
        .expect("EMAIL_KEY must be set");
    let sender = EmailNameData {
        email: "no-reply@bjustcoin.com".to_string(),
        name: "BJustCoin Team".to_string(),
    }; 
    let recipient = EmailUserReq {
        name: body.name.clone(),
        email: body.email.clone(),
    };

    let data = Personalization {
        "personalizations": PersonalizationsData {
            "from": EmailNameData {
                "email": sender.email.clone(),
                "name": sender.name.clone()
            },
            "to": EmailNameData {
                "email": recipient.email.clone(),
                "name": recipient.name.clone()
            },
        },
        "from": EmailNameData {
            "email": sender.email.clone(),
            "name": sender.name.clone()
        },
        "subject": "Bjustcoin - Email confirmation code".to_string(),
        "content": ContentData {
            "type": "text/plain".to_string(),
            "value": token_string.to_string()
        },
    }
    crate::utils::request_post("https://api.sendgrid.com/v3/mail/send".to_string(), &data, api_key);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Verification email sent",
    })))
}

pub async fn logout(session: Session) -> Result<HttpResponse, AuthError> {
    session.clear();
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
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
    session: &Session,
    req: &HttpRequest
) -> Result<HttpResponse, AuthError> {
    use crate::utils::{is_json_request, set_current_user};

    let result = find_user(data);
    let is_json = is_json_request(req);

    match result {
        Ok(user) => {
            set_current_user(&session, &user);
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


pub async fn login(req: HttpRequest, session: Session, data: Json<LoginUser2>) -> Json<AuthResp> {
    if is_signed_in(&session) {
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
        let user_some = User::get_user_with_email(&data.email); 
        if user_some.is_ok() {
            let _new_user = user_some.expect("E.");
            handle_sign_in(data, &session, &req);
            return Json(AuthResp {
                id:         _new_user.id,
                first_name: _new_user.first_name.clone(),
                last_name:  _new_user.last_name.clone(),
                email:      _new_user.email.clone(),
                perm:       _new_user.perm,
                image:      _new_user.image.clone(),
                phone:      _new_user.phone,
            });
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

pub async fn process_signup(session: Session, data: Json<NewUserJson>) -> Json<AuthResp> {
    if is_signed_in(&session) {
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

        let _new_user = User::create(data);

        let _session_user = SessionUser {
            id:    _new_user.id,
            email: _new_user.email.clone(),
        };

        crate::utils::set_current_user(&session, &_session_user);
        return Json(AuthResp {
            id:         _new_user.id,
            first_name: _new_user.first_name.clone(),
            last_name:  _new_user.last_name.clone(),
            email:      _new_user.email.clone(),
            perm:       _new_user.perm,
            image:      _new_user.image,
            phone:      None,
        })
    }
}

pub async fn process_reset(session: Session, data: Json<NewPasswordJson>) -> Json<AuthResp> {
    if is_signed_in(&session) {
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

            crate::utils::set_current_user(&session, &_session_user);
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