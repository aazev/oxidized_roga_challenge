use std::{str::FromStr, sync::Arc};

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
    State(app_state): State<Arc<ApplicationState>>,
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
    println!("auth_header: {}", auth_header);
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
    let user = UserModel::get_by_uuid(user_uuid, &app_state.database_connection).await;
    println!("User: {:?}", user);
    match user {
        Ok(user) => {
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
