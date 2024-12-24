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
use crate::utils::{establish_connection, get_limit_offset, NewUserForm};
use crate::errors::Error;
use actix_web::web::Json;
use crate::views::{NewUserJson, AuthResp};

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
    pub password:   String,
    pub perm:       i16,
    pub phone:      Option<String>,
    pub created:    NaiveDateTime,
    pub image:      Option<String>,
}

impl User {
    pub fn get_count_users(&self) -> usize {
        return schema::users::table
            .filter(schema::users::perm.eq(vec!(USER, USERCANBUYTOCKEN)))
            .select(schema::users::id)
            .load::<i32>(&_connection)
            .expect("E.");
    }
    pub fn get_users(&self, limit: i64, offset: i64) -> Json<Vec<AuthResp>> {
        return Json(schema::users::table
            .filter(schema::users::perm.eq_any(vec!(USER, USERCANBUYTOCKEN)))
            .order(schema::users::created.desc())
            .limit(limit)
            .offset(offset)
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::email,
                schema::users::image.nullable(),
                schema::users::phone.nullable(),
            ))
            .load::<AuthResp>(&_connection)
            .expect("E."));
    }
    pub fn get_users_list(&self, page: i32, limit: Option<i64>) -> (Json<Vec<AuthResp>, i32) {
        let _limit = get_limit(limit, 20);
        if !self.is_admin() {
            return Ok(Json(AuthResp { 
                id:         0,
                first_name: "".to_string(),
                last_name:  "".to_string(),
                email:      "".to_string(),
                perm:       0,
                image:      None,
                phone:      None,
            }), 0);
        }
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<AuthResp>;

        if page > 1 {
            let step = (page - 1) * _limit;
            have_next = page * _limit + 1;
            object_list = User::get_users(_limit.into(), step.into())?;
        }
        else {
            have_next = _limit + 1;
            object_list = User::get_users(_limit.into(), 0)?;
        }
        if User::get_users(1, have_next.into())?.len() > 0 {
            next_page_number = page + 1;
        }
        let _tuple = (object_list, next_page_number);
        Ok(Json(_tuple))
    }

    pub fn is_superuser(&self) -> bool {
        return self.perm == SUPERUSER;
    }
    pub fn is_admin(&self) -> bool {
        return self.perm == ADMIN | self.perm == SUPERUSER;
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
            password:   crate::utils::hash_password(&form.password),
            perm:       1,
            phone:      None,
            created:    chrono::Utc::now().naive_utc(),
            image:      None,
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
    pub created:    NaiveDateTime,
    pub image:      Option<String>,
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