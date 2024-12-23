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
};
use serde::{Serialize, Deserialize};
use crate::utils::{establish_connection, NewUserForm};
use crate::errors::Error;
use actix_web::web::Json;
use crate::views::NewUserJson;

const USER: i16 = 1;
const USERCANBUYTOCKEN: i16 = 2;
const USERISBLOCK: i16 = 5;
const ADMIN: i16 = 50;
const ADMINISBLOCK: i16 = 55;
const SUPERUSER: i16 = 60;

#[derive(Debug, PartialEq)]
enum UserRole {
    USER,
    USERCANBUYTOCKEN,
    USERISBLOCK,
    ADMINISBLOCK,
    ADMIN,
    SUPERUSER,
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct User {
    pub id:         i32,
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
    pub password:   String,
    pub perm:       UserRole,
    pub phone:      Option<String>,
}

impl User {
    pub fn is_superuser(&self) -> bool {
        return self.perm == UserRole::SUPERUSER;
    }
    pub fn is_admin(&self) -> bool {
        return self.perm == UserRole::ADMIN;
    }
    pub fn is_user_in_block(&self) -> bool {
        return self.perm == UserRole::USERISBLOCK;
    }
    pub fn is_admin_in_block(&self) -> bool {
        return self.perm == UserRole::ADMINISBLOCK;
    }
    pub fn is_user_can_buy_tockens(&self) -> bool {
        return self.perm == UserRole::USERCANBUYTOCKEN;
    }

    pub fn create_admin_block(&self, user_id: i32) -> Result<(), Error> {
        if !self.is_superuser() {
            return Err(Error::BadRequest("403"));
        }
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(UserRole::ADMINISBLOCK))
                .execute(&_connection);
        }))
    }
    pub fn delete_admin_block(&self, user_id: i32) -> Result<(), Error> {
        if !self.is_superuser() {
            return Err(Error::BadRequest("403"));
        }
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(UserRole::ADMIN))
                .execute(&_connection);
        }))
    }
    pub fn create_user_block(&self, user_id: i32) -> Result<(), Error> {
        if !self.is_admin() {
            return Err(Error::BadRequest("403"));
        }
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(UserRole::USERISBLOCK))
                .execute(&_connection);
        }))
    }
    pub fn delete_user_block(&self, user_id: i32) -> Result<(), Error> {
        if !self.is_admin() {
            return Err(Error::BadRequest("403"));
        }
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(UserRole::USER))
                .execute(&_connection);
        }))
    }
    pub fn create_can_buy_token(&self, user_id: i32) -> Result<(), Error> {
        if !self.is_superuser() {
            return Err(Error::BadRequest("403"));
        }
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(UserRole::USERCANBUYTOCKEN))
                .execute(&_connection);
        }))
    }
    pub fn delete_can_buy_token(&self, user_id: i32) -> Result<(), Error> {
        if !self.is_superuser() {
            return Err(Error::BadRequest("403"));
        }
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(UserRole::USER))
                .execute(&_connection);
        }))
    }
    pub fn create_admin(&self, user_id: i32) -> Result<(), Error> {
        if !self.is_superuser() {
            return Err(Error::BadRequest("403"));
        }
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(UserRole::ADMIN))
                .execute(&_connection);
        }))
    }
    pub fn delete_admin(&self, user_id: i32) -> Result<(), Error> {
        if !self.is_superuser() {
            return Err(Error::BadRequest("403"));
        }
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(UserRole::USER))
                .execute(&_connection);
        }))
    }
    pub fn create_superuser(user_id: i32) -> Result<(), Error> {
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(UserRole::SUPERUSER))
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
            password:   crate::utils::hash_password(&form.password),
            perm:       1,
            phone:      None,
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
    pub password:   String,
    pub perm:       i16,
    pub phone:      Option<String>,
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