use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use database::{
    models::user::{NewUserModel, UserModel},
    traits::database::Database,
    traits::persist::Persist,
};

use crate::{messages::ok::OkMessage, state::ApplicationState};

pub fn get_router() -> Router<&'static ApplicationState, Body> {
    Router::new()
        .route("/users", get(list_users))
        .route("/users/:id", get(get_user))
        .route("/users", post(create_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
}

pub async fn list_users<'long>(
    State(state): State<&'long ApplicationState>,
) -> Result<Json<Vec<UserModel>>, (StatusCode, String)> {
    let users = UserModel::list(&state.database_connection).await;
    match users {
        Ok(users) => Ok(Json(users)),
        Err(error) => Err((StatusCode::INTERNAL_SERVER_ERROR, error.to_string())),
    }
}

async fn get_user<'long>(
    State(state): State<&'long ApplicationState>,
    Path(id): Path<u64>,
) -> Result<Json<UserModel>, (StatusCode, String)> {
    let user = UserModel::get(id, &state.database_connection).await;
    match user {
        Ok(user) => Ok(Json(user)),
        Err(error) => Err((StatusCode::INTERNAL_SERVER_ERROR, error.to_string())),
    }
}

async fn create_user<'long>(
    State(state): State<&'long ApplicationState>,
    Json(user): Json<NewUserModel>,
) -> Result<Json<UserModel>, (StatusCode, String)> {
    let user = match UserModel::try_from(user) {
        Ok(user) => user,
        Err(error) => return Err((StatusCode::BAD_REQUEST, error.to_string())),
    };

    match user.insert(&state.database_connection).await {
        Ok(user) => Ok(Json(user)),
        Err(error) => Err((StatusCode::INTERNAL_SERVER_ERROR, error.to_string())),
    }
}

async fn update_user<'long>(
    State(state): State<&'long ApplicationState>,
    Json(user): Json<UserModel>,
) -> Result<Json<UserModel>, (StatusCode, String)> {
    match user.update(&state.database_connection).await {
        Ok(user) => Ok(Json(user)),
        Err(error) => Err((StatusCode::INTERNAL_SERVER_ERROR, error.to_string())),
    }
}

async fn delete_user<'long>(
    State(state): State<&'long ApplicationState>,
    Path(id): Path<u64>,
) -> Result<Json<OkMessage>, (StatusCode, String)> {
    // First, get the user from the database.
    let user = match UserModel::get(id, &state.database_connection).await {
        Ok(user) => user,
        Err(error) => return Err((StatusCode::INTERNAL_SERVER_ERROR, error.to_string())),
    };

    // Then, try to delete the user.
    match user.delete(&state.database_connection).await {
        Ok(_) => Ok(Json(OkMessage::new(
            200,
            "User deleted successfully".to_string(),
        ))),
        Err(error) => Err((StatusCode::INTERNAL_SERVER_ERROR, error.to_string())),
    }
}
