
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
use rocket_cors;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate validator_derive;

use dotenv::dotenv;

mod common;
mod routes;
mod db;
mod models;

use rocket_contrib::json::JsonValue;
use rocket_cors::Cors;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn cors_fairing() -> Cors {
    Cors::from_options(&Default::default()).expect("Cors fairing cannot be created")
}

fn rocket_up() -> rocket::Rocket {
    dotenv().ok();
    rocket::custom(common::config::from_env())
        .mount(
            "/",
            routes![
                routes::article::index,
                routes::article::get_article,
                routes::article::api_get_article_by_slug,
                routes::article::api_get_all_articles,
                routes::market::get_market,
                ],
        )
        .attach(db::Conn::fairing())
        .attach(cors_fairing())
        // .attach(config::AppState::manage())
        .register(catchers![not_found])
}

fn main() {
    rocket_up().launch();
}
