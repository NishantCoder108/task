use axum::{
    Json, Router,
    extract::Path,
    routing::{get, post},
    serve,
};
use serde::{Deserialize, Serialize};

async fn home() -> String {
    "Home url".to_string()
}

#[derive(Deserialize)]
struct QuestionItem {
    question_id: u64,
    comment_id: u64,
}

#[derive(Deserialize, Serialize)]
struct CreateUser {
    username: String,
    firstname: String,
}

#[derive(Deserialize, Serialize)]
struct ResponseUser {
    username: String,
    message: String,
}
// async fn get_question(Path((a, b)): Path<(u64, u64)>) -> String {
//     format!("Question Id : {b}, Comment Id: {a}")
// }

async fn get_question(Path(params): Path<QuestionItem>) -> String {
    format!(
        "Question Id: {} , Comment Id: {}",
        params.question_id, params.comment_id
    )
}

async fn create_user(Json(user): Json<CreateUser>) -> Json<ResponseUser> {
    // format!(
    //     "Created User, Firstname: {}, Username: {}",
    //     user.firstname, user.username
    // )

    let user = ResponseUser {
        message: "User created successfully".to_string(),
        username: user.username,
    };

    Json(user)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route(
            "/question/{question_id}/comment/{comment_id}",
            get(get_question),
        )
        .route("/create_user", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    serve(listener, app).await.unwrap();
}
