use std::sync::Arc;

use axum::{extract::State, routing::post, Json, Router};
use database::{
    models::user::{LoginModel, UserModel},
    traits::login::Login,
};
use hyper::{Body, StatusCode};

use crate::{messages::GenericMessage, state::ApplicationState};

pub fn get_router() -> Router<Arc<ApplicationState>, Body> {
    Router::new().route("/users/login", post(login))
}

async fn login(
    State(state): State<Arc<ApplicationState>>,
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
