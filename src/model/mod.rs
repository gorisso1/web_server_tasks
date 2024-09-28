use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: i64,
    pub creator: String,
    pub text: String,
}


pub async  fn create_task_table(pool: &SqlitePool) {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            creator TEXT NOT NULL,
            text TEXT NOT NULL
        );
        "#
    )
        .execute(pool)
        .await
        .expect("Ошибка создания таблицы");
}