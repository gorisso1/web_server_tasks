use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use warp::{reject, Filter};
use warp::reject::Reject;
use crate::model::Task;

#[derive(Deserialize)]
pub struct NewTask {
    pub creator: String,
    pub text: String,
}

#[derive(Debug)]
struct CustomError(String);


impl Reject for CustomError {}




pub async fn create_task(pool: SqlitePool, new_task: NewTask) -> Result<impl warp::Reply, warp::Rejection> {
    sqlx::query("INSERT INTO tasks (creator, text) VALUES (?, ?)")
        .bind(new_task.creator)
        .bind(new_task.text)
        .execute(&pool)
        .await
        .map_err(|_| reject::custom(CustomError("Ошибка создания задачи".into())))?;

    Ok(warp::reply::with_status("Задача создана", warp::http::StatusCode::CREATED))
}

pub async fn get_tasks(pool: SqlitePool) -> Result<impl warp::Reply, warp::Rejection> {
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(&pool)
        .await
        .map_err(|_| reject::custom(CustomError("Ошибка создания задачи".into())))?;

    Ok(warp::reply::json(&tasks))
}