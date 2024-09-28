use sqlx::sqlite::SqlitePool;
use dotenv::dotenv;
use std::env;
use warp::Filter;

mod model;
mod controllers;



#[tokio::main]
async fn main() {
     dotenv().ok();

     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
     let pool = SqlitePool::connect(&database_url).await.expect("Ошибка соединение с базой данных!");

     model::create_task_table(&pool).await;


     let pool_filter = warp::any().map(move || pool.clone());

     let create_task_route = warp::post()
         .and(warp::path("tasks"))
         .and(warp::body::json())
         .and(pool_filter.clone())
         .and_then(|new_task, pool| controllers::create_task(pool, new_task));

     let get_tasks_route = warp::get()
         .and(warp::path("tasks"))
         .and(pool_filter.clone())
         .and_then(controllers::get_tasks);

     let routes = create_task_route.or(get_tasks_route);

     warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;






}

