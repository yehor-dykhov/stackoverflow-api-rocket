#[macro_use]
extern crate rocket;

mod cors;
mod handlers;
mod persistence;
mod models;

use sqlx::postgres::PgPoolOptions;
use cors::*;
use handlers::*;
use crate::models::Question;

#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenvy::dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to create Postgres connection pool!");

    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                read_questions,
                delete_question,
                create_answer,
                read_answers,
                delete_answer
            ],
        )
        .attach(CORS)
}
