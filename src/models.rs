use super::schema::workers;

use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use diesel::data_types::PgTimestamp;

#[derive(Queryable, Serialize)]
pub struct Worker {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime
}

#[derive(Insertable, Deserialize)]
#[table_name="workers"]
pub struct NewWorker {
    pub name: String,
    pub email: String,
}