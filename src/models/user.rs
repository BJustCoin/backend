use crate::schema;
use crate::schema::{
    users,
    new_wallets,
    new_white_lists,
};
use crate::diesel::{
    Queryable,
    Insertable,
    QueryDsl,
    ExpressionMethods,
    RunQueryDsl,
    Connection,
};
use serde::{Serialize, Deserialize};
use crate::utils::{establish_connection, get_limit};
use crate::errors::Error;
use actix_web::web::Json;
use crate::views::{NewUserJson, AuthResp, AuthRespData};

const USER: i16 = 1;
const USERCANBUYTOCKEN: i16 = 2;
const USERISBLOCK: i16 = 5;
const ADMIN: i16 = 50;
const ADMINISBLOCK: i16 = 55;
const SUPERUSER: i16 = 60;

#[derive(Deserialize, Serialize, Debug, Queryable)]
pub struct UserData {
    pub id:         i32,
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
    pub perm:       i16,
    pub image:      Option<String>,
    pub phone:      Option<String>,
} 

impl UserData {
    pub fn get_user_wallets(&self) -> Vec<UserWallet> {
        let _connection = establish_connection();
        return schema::suggest_items::table 
            .filter(schema::suggest_items::email.eq(self.email.clone()))
            .filter(schema::suggest_items::status.eq(1))
            .select((
                schema::suggest_items::address,
                schema::suggest_items::tokens,
                schema::suggest_items::token_type
            ))
            .load::<crate::models::UserWallet>(&_connection)
            .expect("E.");
    } 
}


#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct User {
    pub id:         i32,
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
    pub phone:      Option<String>,
    pub password:   String,
    pub perm:       i16,
    pub image:      Option<String>,
    pub created:    chrono::NaiveDateTime,
    pub uuid:       String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SmallUsers {
    pub users: Vec<SmallUser>,
}  
#[derive(Deserialize, Queryable, Serialize, Debug)]
pub struct SmallUser {
    pub id:         i32,
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
}

impl User {
    pub fn get_user_wallets(&self) -> Vec<UserWallet> {
        let _connection = establish_connection();
        return schema::suggest_items::table 
            .filter(schema::suggest_items::email.eq(self.email.clone()))
            .filter(schema::suggest_items::status.eq(1))
            .select(( 
                schema::suggest_items::address,
                schema::suggest_items::tokens,
                schema::suggest_items::token_type
            ))
            .load::<crate::models::UserWallet>(&_connection)
            .expect("E.");
    } 
    pub fn get_small_users() -> Vec<SmallUser> {
        let _connection = establish_connection();
        return  schema::users::table
            .filter(schema::users::perm.eq_any(vec!(USER, USERCANBUYTOCKEN)))
            .order(schema::users::created.desc())
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::email,
            )) 
            .load::<SmallUser>(&_connection)
            .expect("E.");
    }
    pub fn get_users(limit: i64, offset: i64) -> Vec<AuthResp> {
        let _connection = establish_connection();
        let _users = schema::users::table
            .filter(schema::users::perm.eq_any(vec!(USER, USERCANBUYTOCKEN)))
            .order(schema::users::created.desc())
            .limit(limit)
            .offset(offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::email,
                schema::users::perm,
                schema::users::image,
                schema::users::phone,
            )) 
            .load::<UserData>(&_connection)
            .expect("E.");
        let mut stack = Vec::new();
        for u in _users.iter() {
            stack.push (
                AuthResp {
                    id:         u.id,
                    first_name: u.first_name.clone(),
                    last_name:  u.last_name.clone(),
                    email:      u.email.clone(),
                    perm:       u.perm,
                    image:      u.image.clone(),
                    phone:      u.phone.clone(),
                    white_list: u.get_user_wallets(),
                }
            )
        };
        return stack;
    }
    pub fn get_users_list(page: i64, limit: Option<i64>) -> AuthRespData {
        let _limit = get_limit(limit, 20);
        //if !self.is_admin() {
        //    AuthRespData {
        //        data: Vec::new(),
        //        next: 0,
        //    };
        //}
        let mut next_page_number = 0;
        let have_next: i64;
        let object_list: Vec<AuthResp>;

        if page > 1 {
            let step = (page - 1) * _limit;
            have_next = page * _limit + 1;
            object_list = User::get_users(_limit.into(), step.into());
        }
        else {
            have_next = _limit + 1;
            object_list = User::get_users(_limit.into(), 0);
        }
        if User::get_users(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }
        AuthRespData {
            data: object_list,
            next: next_page_number,
        }
    }

    pub fn get_admins(limit: i64, offset: i64) -> Vec<AuthResp> {
        let _connection = establish_connection();
        let _users = schema::users::table
            .filter(schema::users::perm.eq(ADMIN))
            .order(schema::users::created.desc())
            .limit(limit)
            .offset(offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::email,
                schema::users::perm,
                schema::users::image,
                schema::users::phone,
            )) 
            .load::<UserData>(&_connection)
            .expect("E.");

        let mut stack = Vec::new();
        for u in _users.iter() {
            stack.push (
                AuthResp {
                    id:         u.id,
                    first_name: u.first_name.clone(),
                    last_name:  u.last_name.clone(),
                    email:      u.email.clone(),
                    perm:       u.perm,
                    image:      u.image.clone(),
                    phone:      u.phone.clone(),
                    white_list: u.get_user_wallets(),
                }
            )
        };
        return stack;
    }
    pub fn get_admins_list(page: i64, limit: Option<i64>) -> AuthRespData {
        let _limit = get_limit(limit, 20);
        let mut next_page_number = 0;
        let have_next: i64;
        let object_list: Vec<AuthResp>;

        if page > 1 {
            let step = (page - 1) * _limit;
            have_next = page * _limit + 1;
            object_list = User::get_admins(_limit.into(), step.into());
        }
        else {
            have_next = _limit + 1;
            object_list = User::get_admins(_limit.into(), 0);
        }
        if User::get_admins(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }
        AuthRespData {
            data: object_list,
            next: next_page_number,
        }
    }

    pub fn get_banned_users(limit: i64, offset: i64) -> Vec<AuthResp> {
        let _connection = establish_connection();
        let _users = schema::users::table
            .filter(schema::users::perm.eq(USERISBLOCK))
            .order(schema::users::created.desc())
            .limit(limit)
            .offset(offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::email,
                schema::users::perm,
                schema::users::image,
                schema::users::phone,
            )) 
            .load::<UserData>(&_connection)
            .expect("E.");

        let mut stack = Vec::new();
        for u in _users.iter() {
            stack.push (
                AuthResp {
                    id:         u.id,
                    first_name: u.first_name.clone(),
                    last_name:  u.last_name.clone(),
                    email:      u.email.clone(),
                    perm:       u.perm,
                    image:      u.image.clone(),
                    phone:      u.phone.clone(),
                    white_list: u.get_user_wallets(),
                }
            )
        };
        return stack;
    }
    pub fn get_banned_users_list(page: i64, limit: Option<i64>) -> AuthRespData {
        let _limit = get_limit(limit, 20);
        let mut next_page_number = 0;
        let have_next: i64;
        let object_list: Vec<AuthResp>;

        if page > 1 {
            let step = (page - 1) * _limit;
            have_next = page * _limit + 1;
            object_list = User::get_banned_users(_limit.into(), step.into());
        }
        else {
            have_next = _limit + 1;
            object_list = User::get_banned_users(_limit.into(), 0);
        }
        if User::get_banned_users(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }
        AuthRespData {
            data: object_list,
            next: next_page_number,
        }
    }

    pub fn get_banned_admins(limit: i64, offset: i64) -> Vec<AuthResp> {
        let _connection = establish_connection();
        let _users = schema::users::table
            .filter(schema::users::perm.eq(ADMINISBLOCK))
            .order(schema::users::created.desc())
            .limit(limit)
            .offset(offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::email,
                schema::users::perm,
                schema::users::image,
                schema::users::phone,
            )) 
            .load::<UserData>(&_connection)
            .expect("E.");

        let mut stack = Vec::new();
        for u in _users.iter() {
            stack.push (
                AuthResp {
                    id:         u.id,
                    first_name: u.first_name.clone(),
                    last_name:  u.last_name.clone(),
                    email:      u.email.clone(),
                    perm:       u.perm,
                    image:      u.image.clone(),
                    phone:      u.phone.clone(),
                    white_list: u.get_user_wallets(),
                }
            )
        };
        return stack;
    }
    pub fn get_banned_admins_list(page: i64, limit: Option<i64>) -> AuthRespData {
        let _limit = get_limit(limit, 20);
        let mut next_page_number = 0;
        let have_next: i64;
        let object_list: Vec<AuthResp>;

        if page > 1 {
            let step = (page - 1) * _limit;
            have_next = page * _limit + 1;
            object_list = User::get_banned_admins(_limit.into(), step.into());
        }
        else {
            have_next = _limit + 1;
            object_list = User::get_banned_admins(_limit.into(), 0);
        }
        if User::get_banned_admins(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }
        AuthRespData {
            data: object_list,
            next: next_page_number,
        }
    }

    pub fn is_superuser(&self) -> bool {
        return self.perm == SUPERUSER;
    }
    pub fn is_admin(&self) -> bool {
        return self.perm == ADMIN || self.is_superuser();
    }
    pub fn is_user_in_block(&self) -> bool {
        return self.perm == USERISBLOCK;
    }
    pub fn is_admin_in_block(&self) -> bool {
        return self.perm == ADMINISBLOCK;
    }

    pub fn create_admin_block(&self, user_id: i32) -> Result<(), Error> {
        if !self.is_superuser() {
            return Err(Error::BadRequest("403".to_string()));
        }
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(ADMINISBLOCK))
                .execute(&_connection);

            crate::models::Log::create({
                Json(crate::models::NewLogJson {
                    user_id:   self.id,
                    text:      "blocked the admin".to_string(),
                    target_id: Some(user_id),
                })
            });
        }))
    }
    pub fn delete_admin_block(&self, user_id: i32) -> Result<(), Error> {
        if !self.is_superuser() {
            return Err(Error::BadRequest("403".to_string()));
        }
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(ADMIN))
                .execute(&_connection);
            crate::models::Log::create({
                Json(crate::models::NewLogJson {
                    user_id:   self.id,
                    text:      "unblocked the admin".to_string(),
                    target_id: Some(user_id),
                })
            });
        }))
    } 
    pub fn create_user_block(&self, user_id: i32) -> Result<(), Error> {
        if !self.is_admin() {
            return Err(Error::BadRequest("403".to_string()));
        }
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(USERISBLOCK))
                .execute(&_connection);
            let _k = NewWhiteList::delete_all(user_id);
            crate::models::Log::create({
                Json(crate::models::NewLogJson {
                    user_id:   self.id,
                    text:      "blocked the user".to_string(),
                    target_id: Some(user_id),
                })
            });
        }))
    }
    pub fn delete_user_block(&self, user_id: i32) -> Result<(), Error> {
        if !self.is_admin() {
            return Err(Error::BadRequest("403".to_string()));
        }
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(USER))
            .execute(&_connection);
            crate::models::Log::create({
                Json(crate::models::NewLogJson {
                    user_id:   self.id,
                    text:      "unblocked the user".to_string(),
                    target_id: Some(user_id),
                })
            });
        }))
    }

    pub fn get_tokenomic_type(types: i16) -> String {
        return match types {
            0 =>  "Strategic".to_string(),
            1 =>  "Seed".to_string(),
            2 =>  "Private Sale".to_string(),
            3 =>  "IDO".to_string(),
            4 =>  "Public Sale".to_string(),
            5 =>  "Advisors".to_string(),
            6 =>  "Team".to_string(),
            7 =>  "Future Team".to_string(),
            8 =>  "Incetives".to_string(),
            9 =>  "Liquidity".to_string(),
            10 => "Ecosystem".to_string(),
            11 => "Loyalty".to_string(),
            _ =>  "Uncnown stage".to_string(),
        }
    }
    pub fn get_full_name(&self) -> String {
        return self.first_name.clone() + &" ".to_string() + &self.last_name;
    }

    pub fn create_admin(&self, user_id: i32) -> Result<(), Error> {
        if !self.is_superuser() {
            return Err(Error::BadRequest("403".to_string()));
        }
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(ADMIN))
                .execute(&_connection);
        }))
    }
    pub fn delete_admin(&self, user_id: i32) -> Result<(), Error> {
        if !self.is_superuser() {
            return Err(Error::BadRequest("403".to_string()));
        }
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(USER))
                .execute(&_connection);
        }))
    }
    pub fn create_superuser(user_id: i32) -> Result<(), Error> {
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(SUPERUSER))
                .execute(&_connection);
        }))
    }

    pub fn get_user_with_email(email: &String, password: &String) -> Result<User, Error> {
        let _connection = establish_connection();
        let _password = crate::utils::hash_password(password);
        return Ok(schema::users::table
            .filter(schema::users::email.eq(email))
            .filter(schema::users::password.eq(_password))
            .first::<User>(&_connection)?);
    }
    pub fn create(form: Json<NewUserJson>) -> User {
        use rand::Rng;

        let id = rand::thread_rng().gen::<[u8; 32]>().to_vec();
        let uuid = hex::encode(id);
        let _connection = establish_connection();
        let form_user = NewUser {
            first_name: form.first_name.clone(),
            last_name:  form.last_name.clone(),
            email:      form.email.clone(),
            phone:      None,
            password:   crate::utils::hash_password(&form.password),
            perm:       1,
            image:      None,
            created:    chrono::Utc::now().naive_utc(),
            uuid:       uuid,
        };

        let _new_user = diesel::insert_into(schema::users::table)
            .values(&form_user)
            .get_result::<User>(&_connection)
            .expect("Error saving user.");
        
        if _new_user.id == 1 {
            diesel::update(&_new_user)
                .set(schema::users::perm.eq(60))
                .execute(&_connection)
                .expect("Error.");
        }
        crate::models::Log::create({
            Json(crate::models::NewLogJson {
                user_id:   _new_user.id,
                text:      "created a profile".to_string(),
                target_id: None,
            })
        });
        return _new_user;
    }

    pub fn get_small_user(user_id: i32) -> SmallUser {
        let _connection = establish_connection();
        return schema::users::table
            .filter(schema::users::id.eq(user_id))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::email,
            ))
            .first::<SmallUser>(&_connection)
            .expect("E.");
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
    pub phone:      Option<String>,
    pub password:   String,
    pub perm:       i16,
    pub image:      Option<String>,
    pub created:    chrono::NaiveDateTime,
    pub uuid:       String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserChange {
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionUser {
    pub id:    i32,
    pub email: String,
}


#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct UserWallet {
    pub wallet:     String,
    pub tokens:     String,
    pub token_type: i16,
} 

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct NewWallet {
    pub id:      i32,
    pub user_id: i32,
    pub link:    String,
}


impl NewWallet {
    pub fn create(user_id: i32, link: String, token_type: i16) -> () {
        let _connection = establish_connection();
        if schema::new_wallets::table
            .filter(schema::new_wallets::link.eq(&link))
            .select(schema::new_wallets::id) 
            .load::<i32>(&_connection)
            .is_err() {
                let form_wallet = NewNewWallet {
                    user_id: user_id,
                    link:    link.clone(),
                };
        
                let _new_wallet = diesel::insert_into(schema::new_wallets::table)
                    .values(&form_wallet)
                    .get_result::<NewWallet>(&_connection)
                    .expect("Error saving wallet.");
        }
        NewWhiteList::create(user_id, token_type);
    }
    pub fn delete(id: i32) -> () {
        let _connection = establish_connection();
        diesel::delete (
            schema::new_wallets::table
                .filter(schema::new_wallets::id.eq(id))
        )
        .execute(&_connection)
        .expect("E");
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="new_wallets"]
pub struct NewNewWallet {
    pub user_id: i32,
    pub link:    String,
}


/*
token_type

1.	Strategic
2.	Seed
3.	Private Sale
4.	IDO
5.	Public Sale
6.	Advisors
7.	Team
8.	Future Team
9.	Incetives
10.	Liquidity
11.	Ecosystem
12.	Loyalty
*/


#[derive(Debug, Queryable, Deserialize, Serialize, Identifiable)]
pub struct NewWhiteList {
    pub id:         i32,
    pub user_id:    i32,
    pub token_type: i16,
}

impl NewWhiteList { 
    pub fn create(user_id: i32, token_type: i16) -> () {
        let _connection = establish_connection();
        if schema::new_white_lists::table
            .filter(schema::new_white_lists::user_id.eq(user_id))
            .filter(schema::new_white_lists::token_type.eq(token_type))
            .select(schema::new_white_lists::id) 
            .load::<i32>(&_connection)
            .is_err() 
        {
            let form_white_lists = NewNewWhiteList {
                user_id:    user_id,
                token_type: token_type.clone(),
            }; 
            let _new_white_lists = diesel::insert_into(schema::new_white_lists::table)
                .values(&form_white_lists)
                .get_result::<NewWhiteList>(&_connection)
                .expect("Error saving white list item.");
        }
    }
    pub fn delete(user_id: i32, token_type: i16) -> () {
        let _connection = establish_connection();
        diesel::delete (
            schema::new_white_lists::table
                .filter(schema::new_white_lists::user_id.eq(user_id))
                .filter(schema::new_white_lists::token_type.eq(token_type))
        )
        .execute(&_connection)
        .expect("E");
    }
    pub fn delete_all(user_id: i32) -> () {
        let _connection = establish_connection();
        diesel::delete (
            schema::new_white_lists::table
                .filter(schema::new_white_lists::user_id.eq(user_id))
        )
        .execute(&_connection)
        .expect("E");
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="new_white_lists"]
pub struct NewNewWhiteList {
    pub user_id:    i32,
    pub token_type: i16,
}