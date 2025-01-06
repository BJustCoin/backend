#[macro_use]
extern crate diesel;
use dotenv::dotenv;

pub mod schema;
pub mod models;
pub mod routes;
mod errors;
mod api_error;
mod vars;
 
use actix_web::{
    HttpServer,
    App,
    middleware::Compress,
    cookie::Key,
};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use crate::routes::routes;
use actix_cors::Cors; 

#[macro_use]
mod utils;
#[macro_use]
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    //let server = websocket::Server::new().start();
    let secret_key = Key::generate();

    HttpServer::new(move || {
        let cors = Cors::default() 
            .allowed_origin("https://bjustcoin.com")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().ends_with(b".bjustcoin.com")
            })
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);
        App::new() 
            .wrap(Compress::default())
            .wrap(cors)
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false)
                    .build(),
            )
            //.data(server.clone())
            .configure(routes)
    })
    .bind("69.167.186.207:9330")?
    .run()
    .await
}