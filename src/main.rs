#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

mod auth;
mod models;
mod schema;

use auth::BasicAuth;
use rocket_contrib::json::{Json, JsonValue};
use rocket::response::status;
use diesel::prelude::*;
use schema::*;
use models::*;

#[database("database_url")]
struct DbConn(diesel::PgConnection);

#[get("/workers")]
async fn get_workers(auth: BasicAuth, conn: DbConn) -> JsonValue {
    conn.run(|c| {
        let all = workers::table.limit(100).load::<Worker>(c).expect("Error");
        json!(all)
    }).await
}

#[get("/workers/<id>")]
fn view_worker(id: i32) -> JsonValue {
    json!({"id": id, "first_name": "Adrian"})
}

#[post("/workers", format = "json", data="<new_worker>")]
async fn create_worker(auth: BasicAuth, conn: DbConn, new_worker: Json<NewWorker>) -> JsonValue {
    conn.run(|c| {
        let result = diesel::insert_into(workers::table)
            .values(new_worker.into_inner())
            .execute(c)
            .expect("Error adding to DB");
        json!(result)
    }).await
}

#[put("/workers/<id>", format = "json")]
fn update_worker(id: i32) -> JsonValue {
    json!({"id": id, "first_name": "Andrzej"})
}

#[delete("/workers/<id>")]
fn delete_worker(id: i32) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({"error": "Not found"})
}

#[catch(401)]
fn unauthorized() -> JsonValue {
    json!({"error": "Not authorized"})
}

#[rocket::main]
async fn main() {
    let _ = rocket::ignite()
        .mount("/", routes![
            get_workers, 
            view_worker,
            create_worker,
            update_worker,
            delete_worker
        ])
        .register(catchers![
            not_found,
            unauthorized
        ])
        .attach(DbConn::fairing())
        .launch()
        .await;
}