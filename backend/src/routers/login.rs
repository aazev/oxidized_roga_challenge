use axum::{extract::State, routing::post, Json, Router};
use database::{
    models::user::{LoginModel, NewUserModel, UserModel},
    traits::{login::Login, persist::Persist},
};
use hyper::{Body, StatusCode};

use crate::{messages::GenericMessage, state::ApplicationState};

pub fn get_router() -> Router<ApplicationState, Body> {
    Router::new()
        .route("/users/login", post(login))
        .route("/users/new", post(create_user))
}

async fn login(
    State(state): State<ApplicationState>,
    Json(user): Json<LoginModel>,
) -> Result<Json<UserModel>, (StatusCode, Json<GenericMessage>)> {
    match UserModel::login(user, &state.database_connection).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            Json(GenericMessage::new(404, "User not found".to_string())),
        )),
    }
}

async fn create_user(
    State(state): State<ApplicationState>,
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
