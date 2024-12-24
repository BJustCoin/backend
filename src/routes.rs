use actix_web::web;
use crate::views::{
    auth,
    admin_progs,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(auth::auth_routes)
    .configure(admin_progs::admin_routes)
    ;
}
