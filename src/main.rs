use std::{collections::HashMap, fmt::Error, sync::Arc};

use axum::{
    Json, Router,
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    routing::{Route, delete, get, post, put},
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[tokio::main]

async fn main() {
    let state = Arc::new(AppState {
        tasks: Mutex::new(TaskHash {
            data: HashMap::default(),
        }),
    });
    let app = Router::new()
        .route("/", get(async || "home"))
        .route("/task", post(add_task))
        .route("/task", get(read_tasks))
        .route("/task", put(update_task))
        .route("/task/{id}", delete(delete_task))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server is listening : {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn add_task(
    State(task): State<Arc<AppState>>,
    Json(data): Json<TaskState>,
) -> Result<Json<TaskResponse>, (StatusCode, Json<TaskResponse>)> {
    println!("Task's state: {:?}", task.tasks);
    let mut state = task.tasks.lock().await;

    let task = TaskState {
        id: data.id,
        content: data.content,
        created_at: data.created_at,
        updated_at: data.updated_at,
    };

    if !state.data.contains_key(&task.id) {
        state.data.insert(task.id.clone(), task.clone());
        println!("State: {:?}", state);
        Ok(Json(TaskResponse {
            message: "Task has been created successfully".to_string(),
            id: task.id,
        }))
    } else {
        Err((
            StatusCode::CONFLICT,
            Json(TaskResponse {
                message: "Data already exist".to_string(),
                id: task.id,
            }),
        ))
    }
}

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
#[derive(Deserialize, Serialize, Debug)]
struct TaskReadResponse {
    tasks: Vec<TaskState>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct TaskState {
    id: String,
    content: String,
    created_at: String,
    updated_at: String,
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
    id: String,
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
*/
