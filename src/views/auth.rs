use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    web::Json,
};
use crate::api_error::ApiError;
use serde::{Deserialize, Serialize};
use crate::utils::{
    verify,
};
use crate::models::{
    User, 
    SessionUser, 
    UserWallet,
    EmailVerificationToken, 
    EmailVerificationTokenMessage
};
use crate::errors::AuthError;
use chrono::Utc; 


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

#[derive(Debug, Serialize)]
struct EmailResp {
    message:  String,
}
#[derive(Deserialize, Serialize)]
struct EmailUser {
    name: String,
    email: String,
} 

async fn invite(body: web::Json<EmailUserReq>) -> Result<HttpResponse, ApiError> {
    let body = body.into_inner();

    let user_some = User::get_user_with_email(body.email.clone()); 
    if user_some.is_ok() {
        return Ok(HttpResponse::Ok().json(serde_json::json!(
            EmailResp{
                message: "The profile already exists by such mail.".to_string(),
            }
        )));
    }
    let token_data = EmailVerificationTokenMessage {
        id:    None, 
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
    Ok(HttpResponse::Ok().json(serde_json::json!(
        EmailResp{
            message: "Verification email sent".to_string(),
        }
    )));
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

#[derive(Deserialize, Serialize, Debug, Queryable)]
pub struct AuthResp {
    pub id:         i32,
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
    pub perm:       i16,
    pub image:      Option<String>,
    pub phone:      Option<String>,
    pub white_list: Vec<UserWallet>,
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
    pub white_list: Vec<UserWallet>,
}

fn find_user(email: String, password: String) -> Result<User, AuthError> {
    let user_some = User::get_user_with_email(email); 
    if user_some.is_ok() { 
        let _user = user_some.expect("Error.");
        if let Ok(matching) = verify(&_user.password, &password) {
            if matching {
                return Ok(_user);
            }
        }
    }
    Err(AuthError::NotFound(String::from("User not found")))
}

pub async fn login(req: HttpRequest, data: Json<LoginUser2>) -> Json<AuthResp2> {
    let result = find_user(data.email.clone(), data.password.clone());
    let bad_request = crate::models::AuthRequest::get_or_create(data.email.clone());
    if bad_request.count > 99 {
        return Json(AuthResp2 {
            id:         0,
            first_name: "".to_string(),
            last_name:  "".to_string(),
            email:      "".to_string(),
            perm:       5,
            image:      None,
            phone:      None,
            uuid:       "".to_string(),
            white_list: Vec::new(),
        }); 
    }
    match result {
        Ok(_new_user) => {
            return Json(AuthResp2 { 
                id:         _new_user.id,
                first_name: _new_user.first_name.clone(),
                last_name:  _new_user.last_name.clone(),
                email:      _new_user.email.clone(),
                perm:       _new_user.perm,
                image:      _new_user.image.clone(),
                phone:      _new_user.phone.clone(),
                uuid:       _new_user.uuid.clone(),
                white_list: _new_user.get_user_wallets(),
            });   
        },
        Err(err) => {
            bad_request.update();
            return Json(AuthResp2 {
                id:         0,
                first_name: "".to_string(),
                last_name:  "".to_string(),
                email:      "".to_string(),
                perm:       0,
                image:      None,
                phone:      None,
                uuid:       "".to_string(),
                white_list: Vec::new(),
            });      
        },
    }
}

pub async fn process_signup(data: Json<NewUserJson>) -> Json<AuthResp2> {
    let token_id_res = hex::decode(data.token.clone()); 
    if token_id_res.is_err() {
        println!("token decode error!"); 
        return Json(AuthResp2 {
            id:         0,
            first_name: "".to_string(),
            last_name:  "".to_string(),
            email:      "".to_string(),
            perm:       0,
            image:      None,
            phone:      None,
            uuid:       "".to_string(),
            white_list: Vec::new(),
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
                white_list: Vec::new(),
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
                white_list: Vec::new(),
            });
        }

        if &token.email != &data.email {
            return Json(AuthResp2 {
                id:         0,
                first_name: "".to_string(),
                last_name:  "".to_string(),
                email:      "".to_string(),
                perm:       0,
                image:      None,
                phone:      None,
                uuid:       "".to_string(),
                white_list: Vec::new(),
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
                address: "Beatrice.OBrien@justlaw.com",
                name: "Beatrice OBrien",
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
            image:      _new_user.image.clone(),
            phone:      None,
            uuid:       _new_user.uuid.clone(),
            white_list: _new_user.get_user_wallets(),
        })
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewPasswordJson {
    pub email:    String,
    pub password: String,
    pub token:    String,
} 
pub async fn process_reset(data: Json<NewPasswordJson>) -> Json<AuthResp2> {
        let token_id_res = hex::decode(data.token.clone());
        if token_id_res.is_err() {
            println!("hex decode token error");
            return Json(AuthResp2 {
                id:         0,
                first_name: "".to_string(),
                last_name:  "".to_string(),
                email:      "".to_string(),
                perm:       0,
                image:      None,
                phone:      None,
                uuid:       "".to_string(),
                white_list: Vec::new(),
            });
        }
        let token_id = token_id_res.expect("E.");
        
        let token_res = EmailVerificationToken::find(&token_id);
        if token_res.is_err() {
            println!("EmailVerificationToken not found");
            return Json(AuthResp2 {
                id:         0,
                first_name: "".to_string(),
                last_name:  "".to_string(),
                email:      "".to_string(),
                perm:       0,
                image:      None,
                phone:      None,
                uuid:       "".to_string(),
                white_list: Vec::new(),
            });
        }
        let token = token_res.expect("E.");

        if token.email != data.email {
            println!("token.email != data.email");
            return Json(AuthResp2 {
                id:         0,
                first_name: "".to_string(),
                last_name:  "".to_string(),
                email:      "".to_string(),
                perm:       0,
                image:      None,
                phone:      None,
                uuid:       "".to_string(),
                white_list: Vec::new(),
            });
        }

        if token.expires_at < Utc::now().naive_utc() {
            println!("token.expires_at < Utc::now().naive_utc()");
            return Json(AuthResp2 {
                id:         0,
                first_name: "".to_string(),
                last_name:  "".to_string(),
                email:      "".to_string(),
                perm:       0,
                image:      None,
                phone:      None,
                uuid:       "".to_string(),
                white_list: Vec::new(),
            });
        } 

        let result = crate::models::User::get_user_with_email(data.email.clone());

        match result {
            Ok(_new_user) => {
                println!("result ok!");
                _new_user.reset_password(data.password.clone());
                println!("reset_password!");
                return Json(AuthResp2 { 
                    id:         _new_user.id,
                    first_name: _new_user.first_name.clone(),
                    last_name:  _new_user.last_name.clone(),
                    email:      _new_user.email.clone(),
                    perm:       _new_user.perm,
                    image:      _new_user.image.clone(),
                    phone:      _new_user.phone.clone(),
                    uuid:       _new_user.uuid.clone(),
                    white_list: _new_user.get_user_wallets(),
                });   
            },
            Err(err) => {
                println!("user not found!");
                return Json(AuthResp2 {
                    id:         0,
                    first_name: "".to_string(),
                    last_name:  "".to_string(),
                    email:      "".to_string(),
                    perm:       0,
                    image:      None,
                    phone:      None,
                    uuid:       "".to_string(),
                    white_list: Vec::new(),
                });      
            },
        }
}