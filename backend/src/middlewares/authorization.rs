use std::str::FromStr;

use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use database::{models::user::UserModel, traits::token::Token};
use uuid::Uuid;

use crate::{messages::GenericMessage, state::ApplicationState};

pub async fn auth<B>(
    State(state): State<ApplicationState>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, Json<GenericMessage>)> {
    let headers = req.headers().clone();
    let auth_header = match headers.get(header::AUTHORIZATION) {
        Some(auth_header) => {
            let auth_header = auth_header.to_str().unwrap();
            let auth_header = auth_header.replace("Bearer ", "");
            auth_header
        }
        None => {
            return Err((
                StatusCode::FORBIDDEN,
                Json(GenericMessage {
                    status: 403,
                    message: "Unauthorized".to_string(),
                }),
            ))
        }
    };
    let user_uuid = match Uuid::from_str(&auth_header) {
        Ok(user_uuid) => user_uuid,
        Err(_) => {
            return Err((
                StatusCode::FORBIDDEN,
                Json(GenericMessage {
                    status: 403,
                    message: "Unauthorized".to_string(),
                }),
            ))
        }
    };
    if state.user_cached(&auth_header) {
        let user = state.get_user_cache(&auth_header).unwrap();
        req.extensions_mut().insert(user);
        return Ok(next.run(req).await);
    }
    let user = UserModel::get_by_uuid(user_uuid, &state.database_connection).await;
    match user {
        Ok(user) => {
            state.insert_user_cache(&auth_header, &user);
            req.extensions_mut().insert(user);
            Ok(next.run(req).await)
        }
        Err(_) => Err((
            StatusCode::FORBIDDEN,
            Json(GenericMessage {
                status: 403,
                message: "Unauthorized".to_string(),
            }),
        )),
    }
}
