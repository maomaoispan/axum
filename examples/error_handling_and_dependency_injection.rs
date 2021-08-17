//! Example showing how to convert errors into responses and how one might do
//! dependency injection using trait objects.
//!
//! Run with
//!
//! ```not_rust
//! cargo run --example error_handling_and_dependency_injection
//! ```

#![allow(dead_code)]

use axum::{
    async_trait,
    extract::{Extension, Path},
    handler::{get, post},
    response::IntoResponse,
    route,
    routing::RoutingDsl,
    AddExtensionLayer, Json,
};
use bytes::Bytes;
use http::{Response, StatusCode};
use http_body::Full;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{convert::Infallible, net::SocketAddr, sync::Arc};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "error_handling_and_dependency_injection=debug")
    }
    tracing_subscriber::fmt::init();

    // Inject a `UserRepo` into our handlers via a trait object. This could be
    // the live implementation or just a mock for testing.
    let user_repo = Arc::new(ExampleUserRepo) as DynUserRepo;

    // Build our application with some routes
    let app = route("/users/:id", get(users_show))
        .route("/users", post(users_create))
        // Add our `user_repo` to all request's extensions so handlers can access
        // it.
        .layer(AddExtensionLayer::new(user_repo));

    // Run our application
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Handler for `GET /users/:id`.
///
/// Extracts the user repo from request extensions and calls it. `UserRepoError`s
/// are automatically converted into `AppError` which implements `IntoResponse`
/// so it can be returned from handlers directly.
async fn users_show(
    Path(user_id): Path<Uuid>,
    Extension(user_repo): Extension<DynUserRepo>,
) -> Result<Json<User>, AppError> {
    let user = user_repo.find(user_id).await?;

    Ok(user.into())
}

/// Handler for `POST /users`.
async fn users_create(
    Json(params): Json<CreateUser>,
    Extension(user_repo): Extension<DynUserRepo>,
) -> Result<Json<User>, AppError> {
    let user = user_repo.create(params).await?;

    Ok(user.into())
}

/// Our app's top level error type.
enum AppError {
    /// Something went wrong when calling the user repo.
    UserRepo(UserRepoError),
}

/// This makes it possible to use `?` to automatically convert a `UserRepoError`
/// into an `AppError`.
impl From<UserRepoError> for AppError {
    fn from(inner: UserRepoError) -> Self {
        AppError::UserRepo(inner)
    }
}

impl IntoResponse for AppError {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        let (status, error_json) = match self {
            AppError::UserRepo(UserRepoError::NotFound) => {
                (StatusCode::NOT_FOUND, json!("User not found"))
            }
            AppError::UserRepo(UserRepoError::InvalidUsername) => {
                (StatusCode::UNPROCESSABLE_ENTITY, json!("Invalid username"))
            }
        };

        let mut response = Json(json!({
            "error": error_json,
        }))
        .into_response();

        *response.status_mut() = status;

        response
    }
}

/// Example implementation of `UserRepo`.
struct ExampleUserRepo;

#[async_trait]
impl UserRepo for ExampleUserRepo {
    async fn find(&self, _user_id: Uuid) -> Result<User, UserRepoError> {
        unimplemented!()
    }

    async fn create(&self, _params: CreateUser) -> Result<User, UserRepoError> {
        unimplemented!()
    }
}

/// Type alias that makes it easier to extract `UserRepo` trait objects.
type DynUserRepo = Arc<dyn UserRepo + Send + Sync>;

/// A trait that defines things a user repo might support.
#[async_trait]
trait UserRepo {
    /// Loop up a user by their id.
    async fn find(&self, user_id: Uuid) -> Result<User, UserRepoError>;

    /// Create a new user.
    async fn create(&self, params: CreateUser) -> Result<User, UserRepoError>;
}

#[derive(Debug, Serialize)]
struct User {
    id: Uuid,
    username: String,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    username: String,
}

/// Errors that can happen when using the user repo.
#[derive(Debug)]
enum UserRepoError {
    NotFound,
    InvalidUsername,
}
