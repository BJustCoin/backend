use crate::schema;
use crate::schema::{
    users,
};
use crate::diesel::{
    Queryable,
    Insertable,
    QueryDsl,
    ExpressionMethods,
    RunQueryDsl,
    Connection,
    NullableExpressionMethods,
};
use serde::{Serialize, Deserialize};
use crate::utils::{establish_connection, get_limit, NewUserForm};
use crate::errors::Error;
use actix_web::web::Json;
use crate::views::{NewUserJson, AuthResp, AuthRespData};

const USER: i16 = 1;
const USERCANBUYTOCKEN: i16 = 2;
const USERISBLOCK: i16 = 5;
const ADMIN: i16 = 50;
const ADMINISBLOCK: i16 = 55;
const SUPERUSER: i16 = 60;


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
}

impl User {
    pub fn get_users(limit: i64, offset: i64) -> Vec<AuthResp> {
        let _connection = establish_connection();
        return schema::users::table
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
            .load::<AuthResp>(&_connection)
            .expect("E.");
    }
    pub fn get_users_list(&self, page: i64, limit: Option<i64>) -> AuthRespData {
        let _limit = get_limit(limit, 20);
        if !self.is_admin() {
            AuthRespData {
                data: Vec::new(),
                next: 0,
            };
        }
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
        return schema::users::table
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
            .load::<AuthResp>(&_connection)
            .expect("E.");
    }
    pub fn get_admins_list(&self, page: i64, limit: Option<i64>) -> AuthRespData {
        let _limit = get_limit(limit, 20);
        if !self.is_superuser() {
            return AuthRespData {
                data: Vec::new(),
                next: 0,
            };
        }
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
        return schema::users::table
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
            .load::<AuthResp>(&_connection)
            .expect("E.");
    }
    pub fn get_banned_users_list(&self, page: i64, limit: Option<i64>) -> AuthRespData {
        let _limit = get_limit(limit, 20);
        if !self.is_admin() {
            return AuthRespData {
                data: Vec::new(),
                next: 0,
            };
        }
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

    pub fn get_banned_admins(limit: i64, offset: i64) -> Vec<AuthResp> {
        let _connection = establish_connection();
        return schema::users::table
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
            .load::<AuthResp>(&_connection)
            .expect("E.");
    }
    pub fn get_banned_admins_list(&self, page: i64, limit: Option<i64>) -> AuthRespData {
        let _limit = get_limit(limit, 20);
        if !self.is_superuser() {
            return AuthRespData {
                data: Vec::new(),
                next: 0,
            };
        }
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

    pub fn is_superuser(&self) -> bool {
        return self.perm == SUPERUSER;
    }
    pub fn is_admin(&self) -> bool {
        return self.perm == ADMIN || self.perm == SUPERUSER;
    }
    pub fn is_user_in_block(&self) -> bool {
        return self.perm == USERISBLOCK;
    }
    pub fn is_admin_in_block(&self) -> bool {
        return self.perm == ADMINISBLOCK;
    }
    pub fn is_user_can_buy_tockens(&self) -> bool {
        return self.perm == USERCANBUYTOCKEN;
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
        }))
    }
    pub fn create_can_buy_token(&self, user_id: i32) -> Result<(), Error> {
        if !self.is_superuser() {
            return Err(Error::BadRequest("403".to_string()));
        }
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(USERCANBUYTOCKEN))
                .execute(&_connection);
        }))
    }
    pub fn delete_can_buy_token(&self, user_id: i32) -> Result<(), Error> {
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

    pub fn get_user_with_email(email: &String) -> Result<User, Error> {
        let _connection = establish_connection();
        return Ok(schema::users::table
            .filter(schema::users::email.eq(email))
            .first::<User>(&_connection)?);
    }
    pub fn create(form: Json<NewUserJson>) -> User {
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
        return _new_user;
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