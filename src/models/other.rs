use crate::schema;
use crate::schema::{
    logs,
    suggest_items,
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
use crate::models::{SmallUser, User};


#[derive(Debug, Queryable, Deserialize, Serialize, Identifiable)]
pub struct SuggestItem {
    pub id:          i32,
    pub first_name:  String,
    pub middle_name: String,
    pub last_name:   String,
    pub email:       String,
    pub phone:       String,
    pub mobile:      String,
    pub is_agree:    bool,
    pub address:     String,
    pub created:     chrono::NaiveDateTime,
} 

#[derive(Deserialize, Serialize)]
pub struct SuggestRespData {
    pub data: Vec<SuggestItem>,
    pub next: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewSuggestJson {
    pub first_name:  String,
    pub middle_name: String,
    pub last_name:   String,
    pub email:       String,
    pub phone:       String,
    pub mobile:      String,
    pub is_agree:    bool,
    pub address:     String,
}

impl SuggestItem {
    pub fn get(limit: i64, offset: i64) -> Vec<SuggestItem> {
        let _connection = establish_connection();
        return schema::suggest_items::table
            .order(schema::suggest_items::created.desc())
            .limit(limit)
            .offset(offset) 
            .load::<SuggestItem>(&_connection)
            .expect("E.");
    }
    pub fn get_list(page: i64, limit: Option<i64>) -> SuggestRespData {
        let _limit = get_limit(limit, 20);
        let mut next_page_number = 0;
        let have_next: i64;
        let object_list: Vec<SuggestItem>;

        if page > 1 {
            let step = (page - 1) * _limit;
            have_next = page * _limit + 1;
            object_list = SuggestItem::get(_limit.into(), step.into());
        }
        else {
            have_next = _limit + 1;
            object_list = SuggestItem::get(_limit.into(), 0);
        }
        if SuggestItem::get(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }
        SuggestRespData {
            data: object_list,
            next: next_page_number,
        }
    }

    pub fn create(form: Json<NewSuggestJson>) -> () {
        let _connection = establish_connection();
        let form = NewSuggestItem {
            first_name:  form.first_name.clone(),
            middle_name: form.middle_name.clone(),
            last_name:   form.last_name.clone(),
            email:       form.email.clone(),
            phone:       form.phone.clone(),
            mobile:      form.mobile.clone(),
            is_agree:    form.is_agree,
            address:     form.address.clone(),
            created:     chrono::Utc::now().naive_utc(),
        };

        let _new_suggest_item = diesel::insert_into(schema::suggest_items::table)
            .values(&form)
            .execute(&_connection)
            .expect("Error saving suggest form.");
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="suggest_items"]
pub struct NewSuggestItem {
    pub first_name:  String,
    pub middle_name: String,
    pub last_name:   String,
    pub email:       String,
    pub phone:       String,
    pub mobile:      String,
    pub is_agree:    bool,
    pub address:     String,
    pub created:     chrono::NaiveDateTime,
}


#[derive(Debug, Queryable, Deserialize, Serialize, Identifiable)]
pub struct Log {
    pub id:        i32,
    pub user_id:   i32,
    pub text:      String,
    pub created:   chrono::NaiveDateTime,
    pub target_id: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct LogRespData {
    pub data: Vec<LogData>,
    pub next: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewLogJson {
    pub user_id:   i32,
    pub text:      String,
    pub target_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LogData {
    pub user:    SmallUser,
    pub text:    String,
    pub target:  Option<SmallUser>,
    pub created: chrono::NaiveDateTime,
} 

impl Log {
    pub fn get(limit: i64, offset: i64) -> Vec<LogData> {
        let _connection = establish_connection();
        let list = schema::logs::table
            .order(schema::logs::created.desc())
            .limit(limit)
            .offset(offset) 
            .load::<Log>(&_connection)
            .expect("E.");
        let mut stack = Vec::new();
        for i in list.iter() {
            stack.push(i.get_data());
        }
        return stack;
    }
    pub fn get_list(page: i64, limit: Option<i64>) -> LogRespData {
        let _limit = get_limit(limit, 20);
        let mut next_page_number = 0;
        let have_next: i64;
        let object_list: Vec<LogData>;

        if page > 1 {
            let step = (page - 1) * _limit;
            have_next = page * _limit + 1;
            object_list = Log::get(_limit.into(), step.into());
        }
        else {
            have_next = _limit + 1;
            object_list = Log::get(_limit.into(), 0);
        }
        if Log::get(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }
        LogRespData {
            data: object_list,
            next: next_page_number,
        }
    }

    pub fn get_data(&self) -> LogData {
        let _connection = establish_connection();
        let _user = User::get_small_user(self.user_id);
        let target_user: Option<SmallUser>;
        if self.target_id.is_some() {
            target_user = Some(User::get_small_user(self.target_id.unwrap()));
        }
        else {
            target_user = None;
        }
        return LogData {
            user:    _user,
            text:    self.text.clone(),
            target:  target_user,
            created: self.created,
        };
    }

    pub fn get_for_user(user_id: i32, limit: i64, offset: i64) -> Vec<LogData> {
        let _connection = establish_connection();
        let list = schema::logs::table
            .filter(schema::logs::user_id.eq(user_id))
            .order(schema::logs::created.desc())
            .limit(limit)
            .offset(offset) 
            .load::<Log>(&_connection)
            .expect("E.");
        let mut stack = Vec::new();
        for i in list.iter() {
            stack.push(i.get_data());
        }
        return stack;
    }
    pub fn get_list_for_user(user_id: i32, page: i64, limit: Option<i64>) -> LogRespData {
        let _limit = get_limit(limit, 20);
        let mut next_page_number = 0;
        let have_next: i64;
        let object_list: Vec<LogData>;

        if page > 1 {
            let step = (page - 1) * _limit;
            have_next = page * _limit + 1;
            object_list = Log::get_for_user(user_id, _limit.into(), step.into());
        }
        else {
            have_next = _limit + 1;
            object_list = Log::get_for_user(user_id, _limit.into(), 0);
        }
        if Log::get_for_user(user_id, 1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }
        LogRespData {
            data: object_list,
            next: next_page_number,
        }
    }

    pub fn create(form: Json<NewLogJson>) -> () {
        let _connection = establish_connection();
        let form = NewLog {
            user_id:   form.user_id,
            text:      form.text.clone(),
            created:   chrono::Utc::now().naive_utc(),
            target_id: form.target_id,
        };

        let _new_suggest_item = diesel::insert_into(schema::logs::table)
            .values(&form)
            .execute(&_connection)
            .expect("Error saving log.");
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="logs"]
pub struct NewLog {
    pub user_id:   i32,
    pub text:      String,
    pub created:   chrono::NaiveDateTime,
    pub target_id: Option<i32>,
}