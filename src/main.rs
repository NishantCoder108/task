use ::time::PrimitiveDateTime;
use anyhow::Context;
use axum::{
    Error, Json, Router,
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
};
use dotenvy::{dotenv, var};
use serde::{Deserialize, Serialize};
use sqlx::{
    PgPool, Row,
    postgres::{PgPoolOptions, PgRow},
    types::time,
};
use std::{collections::HashMap, time::Duration};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let db_url = var("DATABASE_URL").context("Database url must be set")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_url)
        .await
        .context("Failed to connect to Database URL")?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new()
        .route("/", get(async || "home"))
        .route("/task", post(add_task))
        .route("/task", get(read_tasks))
        .route("/task", put(update_task))
        .route("/task/{id}", delete(delete_task))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("Failed to listen on port: 3000")?;

    println!("Server is listening : {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await?;

    Ok(())
}

#[axum::debug_handler]
async fn add_task(
    State(pool): State<PgPool>,
    Json(data): Json<TaskState>,
) -> Result<Json<TaskResponse>, AppError> {
    // println!("Task's pool state: {:?}", pool);

    let result: PgRow = sqlx::query("INSERT INTO task (title) VALUES ($1) RETURNING id")
        .bind(&data.title)
        .fetch_one(&pool)
        .await
        .context("Failed to insert task")?;

    println!("Insert task: {:?}", result.get::<i32, _>("id"));

    Ok(Json(TaskResponse {
        message: "Task has been created successfully".to_string(),
        id: Some(result.get::<i32, _>("id")),
    }))
}

async fn read_tasks(State(pool): State<PgPool>) -> Result<Json<TaskReadResponse>, AppError> {
    let tasks = sqlx::query("SELECT * FROM task")
        .fetch_all(&pool)
        .await
        .context("Failed to fetch tasks")?;

    let tasks = tasks
        .into_iter()
        .map(|row| TaskStateResponse {
            id: Some(row.get::<i32, _>("id")),
            title: row.get::<String, _>("title"),
            created_at: row.get::<PrimitiveDateTime, _>("created_at"),
            updated_at: row.get::<PrimitiveDateTime, _>("updated_at"),
        })
        .collect();

    Ok(Json(TaskReadResponse { tasks }))
}

async fn update_task(
    State(pool): State<PgPool>,
    Json(data): Json<TaskStateUpdate>,
) -> Result<Json<TaskResponse>, AppError> {
    let task = sqlx::query("UPDATE task SET title = $1 WHERE id = $2 RETURNING id")
        .bind(&data.title)
        .bind(data.id)
        .fetch_one(&pool)
        .await
        .context("Failed to update task")?;

    println!("Update task: {:?}", task.get::<i32, _>("id"));

    Ok(Json(TaskResponse {
        message: "Task has been updated successfully".to_string(),
        id: Some(task.get::<i32, _>("id")),
    }))
}

async fn delete_task(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<TaskResponse>, AppError> {
    /*
    First check if the task exists
    Then delete the task
     */

    let task = sqlx::query("SELECT * FROM task WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .context("No task found with the given ID")?;

    println!("Deleting task: {:?}", task.get::<i32, _>("id"));

    if task.is_empty() {
        // return Err((
        //     StatusCode::NOT_FOUND,
        //     Json(TaskResponse {
        //         message: "Task not found".to_string(),
        //         id: None,
        //     }),
        // ));

        return Err(AppError(anyhow::anyhow!("Task not found")));
    } else {
        // Delete the task
        sqlx::query("DELETE FROM task WHERE id = $1")
            .bind(id)
            .execute(&pool)
            .await
            .context("Failed to delete task")?;

        Ok(Json(TaskResponse {
            message: "Task has been deleted successfully".to_string(),
            id: Some(task.get::<i32, _>("id")),
        }))
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct TaskReadResponse {
    tasks: Vec<TaskStateResponse>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct TaskState {
    id: Option<i32>,
    title: String,
    // created_at: String,
    // updated_at: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct TaskStateUpdate {
    id: i32,
    title: String,
}

#[derive(Serialize, Debug, Deserialize, sqlx::FromRow)]
struct TaskStateResponse {
    id: Option<i32>,
    title: String,
    created_at: PrimitiveDateTime,
    updated_at: PrimitiveDateTime,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
struct TaskHash {
    data: HashMap<String, TaskState>,
}
struct AppState {
    tasks: Mutex<TaskHash>,
}

#[derive(Deserialize, Serialize)]
struct TaskResponse {
    message: String,
    id: Option<i32>,
}

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            // format!("Something went wrong: {}", self),
            Json(TaskResponse {
                message: "Something went wrong".to_string(),
                id: None,
            }),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

/*
1. create a appstate struct
2. create a simple server
3. create a task
4. read a task by postman
5. update the task
6. delete the specific task


7. setup db
8. create , read, update and delete function update

------

1. setup db
2. test db
3. create read update and then delete
*/
