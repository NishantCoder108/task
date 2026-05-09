use dotenvy::{dotenv, var};
use std::{ collections::HashMap,};

use anyhow::Context;
use axum::{
    Error, Json, Router,
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let db_url = var("DATABASE_URL").context("Database url must be set")?;

    // let mut conn = sqlx::postgres::PgConnection::connect(url).await?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .context("Failed to connect to Database URL")?;

    // let pool = sqlx::PgPool::connect(url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?; //add migration to to do

    // let res = sqlx::query("SELECT 'Nishant' as name")
    //     .fetch_one(&pool)
    //     .await?;

    // println!("Test result : {:?}", res);

    // let state = Arc::new(AppState {
    //     tasks: Mutex::new(TaskHash {
    //         data: HashMap::default(),
    //     }),
    // });
    let app = Router::new()
        .route("/", get(async || "home"))
        .route("/task", post(add_task))
        // .route("/task", get(read_tasks))
        // .route("/task", put(update_task))
        // .route("/task/{id}", delete(delete_task))
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
    println!("Task's pool state: {:?}", pool);

    let result = sqlx::query("INSERT INTO task (title) VALUES ($1)")
        .bind(&data.title)
        .execute(&pool)
        .await?;

    println!("Insert result: {:?}", result);

    // let id = result.last_insert_rowid();

    Ok(Json(TaskResponse {
        message: "Task has been created successfully".to_string(),
        id: Some(result.rows_affected() as u64),
    }))
    // let mut state = task.tasks.lock().await;

    // let task = TaskState {
    //     id: None,
    //     title: data.title,
    // };

    // let result = sqlx::query("INSERT INTO task (title) VALUES ($1)")
    //     .bind(&data.title)
    //     .execute(&pool)
    //     .await?;

    // if !state.data.contains_key(&task.id) {
    //     state.data.insert(task.id.clone(), task.clone());
    //     println!("State: {:?}", state);
    //     Ok(Json(TaskResponse {
    //         message: "Task has been created successfully".to_string(),
    //         id: task.id,
    //     }))
    // } else {
    //     Err((
    //         StatusCode::CONFLICT,
    //         Json(TaskResponse {
    //             message: "Data already exist".to_string(),
    //             id: task.id,
    //         }),
    //     ))
    // }

    // Ok(Json(TaskResponse {
    //     message: "Task has been created successfully".to_string(),
    //     id: task.id,
    // }))

    // todo!()
}

/*
async fn read_tasks(State(task): State<Arc<AppState>>) -> Json<TaskReadResponse> {
    let task = task.tasks.lock().await;
    let tasks = task.data.clone();
    println!("Tasks: {:?}", tasks);

    // The field `tasks` in `TaskReadResponse` expects a Vec<TaskState> but `task` is a HashMap<String, TaskState>.
    // You should convert the values of the HashMap into a Vec<TaskState> before returning.
    Json(TaskReadResponse {
        tasks: tasks.values().cloned().collect(),
    })
}

async fn update_task(
    State(state): State<Arc<AppState>>,
    Json(data): Json<TaskState>,
) -> Result<Json<TaskResponse>, (StatusCode, Json<TaskResponse>)> {
    let mut state = state.tasks.lock().await;
    let task = TaskState {
        id: data.id,
        content: data.content,
        created_at: data.created_at,
        updated_at: data.updated_at,
    };

    if state.data.contains_key(&task.id) {
        state.data.insert(task.id.clone(), task.clone());

        Ok(Json(TaskResponse {
            id: task.id,
            message: "Task updated successfully".to_string(),
        }))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(TaskResponse {
                id: task.id,
                message: "Task doesn't exist".to_string(),
            }),
        ))
    }
}

async fn delete_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u32>,
) -> Result<Json<TaskResponse>, Json<TaskResponse>> {
    let mut state = state.tasks.lock().await;

    if !state.data.contains_key(&id.to_string()) {
        Err(Json(TaskResponse {
            id: id.to_string(),
            message: "Task doesn't exist".to_string(),
        }))
    } else {
        let task_data = state.data.remove(&id.to_string());
        println!("Task deketed data: {:?}", task_data);

        Ok(Json(TaskResponse {
            id: id.to_string(),
            message: "Task deleted succesfully.".to_string(),
        }))
    }
}

*/

#[derive(Deserialize, Serialize, Debug)]
struct TaskReadResponse {
    tasks: Vec<TaskState>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct TaskState {
    id: Option<i32>,
    title: String,
    // created_at: String,
    // updated_at: String,
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
    id: Option<u64>,
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
