use axum::{
    Json, Router,
    extract::{Path, Query},
    http::Uri,
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

// localhost:3000/items?page=2&per_page=30

#[derive(Deserialize, Serialize)]
struct Pagination {
    page: Option<u64>,
    per_page: Option<u64>,
}

#[derive(Deserialize, Serialize)]
struct ResponseItems {
    message: String,
    page: u64,
    per_page: u64,
}

async fn list_items(Query(pagination): Query<Pagination>) -> Json<ResponseItems> {
    // format!(
    //     "Page {} have {} items.",
    //     pagination.page.unwrap_or(1),
    //     pagination.per_page.unwrap_or(10)
    // )

    let res = ResponseItems {
        message: "Hey, List items retrived succesfully.".to_string(),
        per_page: pagination.per_page.unwrap_or(20),
        page: pagination.page.unwrap_or(1),
    };

    Json(res)
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
        .route("/create_user", post(create_user))
        .route("/items", get(list_items));

    let url = "http://localhost:3000/items?page=3&per_page=345"
        .parse()
        .unwrap();
    let res: Query<Pagination> = Query::try_from_uri(&url).unwrap();

    println!(
        "Page : {}, Per Page : {}",
        res.page.unwrap(),
        res.per_page.unwrap()
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    serve(listener, app).await.unwrap();
}
