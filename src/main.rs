use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Set up PostgreSQL connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Could not connect to database");

    // Define routes
    let app = Router::new()
        // API routes for CRUD operations
        .route("/api/users", post(create_user).get(get_users))
        .route("/api/users/:id", get(get_user))
        // Fallback to serve static files from the "static" directory for non-API routes
        .fallback(axum::routing::get_service(ServeDir::new("static")).handle_error(|_| async {
            (StatusCode::INTERNAL_SERVER_ERROR, "Static file error")
        }))
        // Attach database connection pool as shared state
        .layer(Extension(pool));

    // Define server address and start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Define User struct for database and API responses
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

// Define CreateUser struct for handling incoming JSON payload
#[derive(Debug, Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

// Handler to create a new user
async fn create_user(
    Json(payload): Json<CreateUser>,
    Extension(pool): Extension<sqlx::PgPool>,
) -> impl IntoResponse {
    // Insert user into the database and return the created user as JSON
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
        payload.name,
        payload.email
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    (StatusCode::CREATED, Json(user))
}

// Handler to get all users
async fn get_users(Extension(pool): Extension<sqlx::PgPool>) -> impl IntoResponse {
    let users = sqlx::query_as!(User, "SELECT id, name, email FROM users")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(users)
}

// Handler to get a single user by ID
async fn get_user(
    Path(id): Path<i32>,
    Extension(pool): Extension<sqlx::PgPool>,
) -> impl IntoResponse {
    match sqlx::query_as!(User, "SELECT id, name, email FROM users WHERE id = $1", id)
        .fetch_one(&pool)
        .await
    {
        Ok(user) => Json(user).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}
