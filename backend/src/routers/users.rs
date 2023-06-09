use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, put},
    Json, Router,
};
use database::{models::user::UserModel, traits::database::Database, traits::persist::Persist};

use crate::{messages::GenericMessage, state::ApplicationState};

pub fn get_router() -> Router<ApplicationState, Body> {
    Router::new()
        .route("/users", get(list_users))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
}

// #[utoipa::path(
//     get,
//     path = "/users",
//     security(
//         ("api_token" = [])
//     ),
//     responses(
//         (status = 200, description="Listed all users successfully", body=[UserModel]),
//         (status = 403, description="Unauthorized", body=GenericMessage),
//     )
// )]
pub async fn list_users(
    State(state): State<ApplicationState>,
) -> Result<Json<Vec<UserModel>>, (StatusCode, String)> {
    let users = UserModel::list(&state.database_connection).await;
    match users {
        Ok(users) => Ok(Json(users)),
        Err(error) => Err((StatusCode::NOT_FOUND, error.to_string())),
    }
}

async fn get_user(
    State(state): State<ApplicationState>,
    Path(id): Path<u64>,
) -> Result<Json<UserModel>, (StatusCode, String)> {
    let user = UserModel::get(id, &state.database_connection).await;
    match user {
        Ok(user) => Ok(Json(user)),
        Err(error) => Err((StatusCode::NOT_FOUND, error.to_string())),
    }
}

async fn update_user(
    State(state): State<ApplicationState>,
    Json(user): Json<UserModel>,
) -> Result<Json<UserModel>, (StatusCode, String)> {
    match user.update(&state.database_connection).await {
        Ok(user) => Ok(Json(user)),
        Err(error) => Err((StatusCode::NOT_FOUND, error.to_string())),
    }
}

async fn delete_user(
    State(state): State<ApplicationState>,
    Path(id): Path<u64>,
) -> Result<Json<GenericMessage>, (StatusCode, String)> {
    // First, get the user from the database.
    let user = match UserModel::get(id, &state.database_connection).await {
        Ok(user) => user,
        Err(error) => return Err((StatusCode::NOT_FOUND, error.to_string())),
    };

    // Then, try to delete the user.
    match user.delete(&state.database_connection).await {
        Ok(_) => Ok(Json(GenericMessage::new(
            200,
            "User deleted successfully".to_string(),
        ))),
        Err(error) => Err((StatusCode::NOT_FOUND, error.to_string())),
    }
}
