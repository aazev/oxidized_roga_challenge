use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use database::{
    models::person::{NewPersonModel, PersonModel},
    traits::database::Database,
    traits::persist::Persist,
};
use hyper::StatusCode;

use crate::{messages::GenericMessage, state::ApplicationState};

pub fn get_router() -> Router<Arc<ApplicationState>, Body> {
    Router::new()
        .route("/persons", get(list_persons))
        .route("/persons/:id", get(get_person))
        .route("/persons", post(create_person))
        .route("/persons/:id", delete(delete_person))
}

pub async fn list_persons(
    State(state): State<Arc<ApplicationState>>,
) -> Result<Json<Vec<PersonModel>>, (StatusCode, Json<GenericMessage>)> {
    let persons = PersonModel::list(&state.database_connection).await;
    match persons {
        Ok(persons) => Ok(Json(persons)),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(GenericMessage::new(
                500,
                "Error listing persons".to_string(),
            )),
        )),
    }
}

pub async fn get_person(
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<u64>,
) -> Result<Json<PersonModel>, (StatusCode, Json<GenericMessage>)> {
    let person = PersonModel::get(id, &state.database_connection).await;
    match person {
        Ok(person) => Ok(Json(person)),
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            Json(GenericMessage::new(404, "Person not found".to_string())),
        )),
    }
}

pub async fn create_person(
    State(state): State<Arc<ApplicationState>>,
    Json(person): Json<NewPersonModel>,
) -> Result<Json<PersonModel>, (StatusCode, Json<GenericMessage>)> {
    let person = match PersonModel::try_from(person) {
        Ok(person) => person,
        Err(error) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(GenericMessage::new(400, error.to_string())),
            ))
        }
    };
    match person.insert(&state.database_connection).await {
        Ok(person) => Ok(Json(person)),
        Err(error) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(GenericMessage::new(500, error.to_string())),
        )),
    }
}

pub async fn delete_person(
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<u64>,
) -> Result<Json<GenericMessage>, (StatusCode, Json<GenericMessage>)> {
    let person = match PersonModel::get(id, &state.database_connection).await {
        Ok(person) => person,
        Err(_) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(GenericMessage::new(404, "Person not found".to_string())),
            ))
        }
    };
    match person.delete(&state.database_connection).await {
        Ok(_) => Ok(Json(GenericMessage::new(
            200,
            "Person deleted successfully".to_string(),
        ))),
        Err(error) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(GenericMessage::new(500, error.to_string())),
        )),
    }
}
