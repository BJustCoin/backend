use dotenv::dotenv;
use std::env::var;

pub fn secret_key() -> String {
  dotenv().ok();
  var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8))
}
