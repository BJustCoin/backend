use actix_web::web;
use crate::views::{
    auth,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(auth::auth_routes)
    ;
}
