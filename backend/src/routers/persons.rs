use crate::{
    objects::{address::Address, annotation::Annotation, person::Person},
    state::ApplicationState,
};
use axum::{
    body::Body,
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use cep_service::{responses::service::CepServiceResponse, structs::cep::Cep};
use database::{
    models::annotation::AnnotationModel,
    models::person::{NewPersonModel, PersonModel},
    traits::database::Database,
    traits::persist::Persist,
};

use hyper::StatusCode;

use crate::messages::GenericMessage;

pub fn get_router() -> Router<ApplicationState, Body> {
    Router::new()
        .route("/persons", get(list_persons))
        .route("/persons/:id", get(get_person))
        .route("/persons", post(create_person))
        .route("/persons/:id", delete(delete_person))
}

pub async fn list_persons(
    State(state): State<ApplicationState>,
) -> Result<Json<Vec<Person>>, (StatusCode, Json<GenericMessage>)> {
    match PersonModel::list(&state.database_connection).await {
        Ok(person_models) => {
            let mut persons = Vec::new();
            for person_model in person_models {
                let cep = match Cep::try_from(person_model.cep.clone()) {
                    Ok(cep) => cep,
                    Err(_) => {
                        continue;
                    }
                };
                let address = state.cep_service.get_address(cep).await;

                persons.push(Person {
                    id: person_model.id,
                    name: person_model.name,
                    mothers_name: person_model.mothers_name,
                    fathers_name: person_model.fathers_name,
                    cep: person_model.cep,
                    address: match address {
                        CepServiceResponse::CepFound(address) => Some(Address {
                            logradouro: address.logradouro.clone(),
                            complemento: address.complemento.clone(),
                            bairro: address.bairro.clone(),
                            localidade: address.localidade.clone(),
                            uf: address.uf.clone(),
                            ibge: address.ibge.clone(),
                            gia: address.gia.clone(),
                            ddd: address.ddd.clone(),
                            siafi: address.siafi.clone(),
                        }),
                        _ => None,
                    },
                    annotations: match AnnotationModel::list(
                        person_model.id,
                        &state.database_connection,
                    )
                    .await
                    {
                        Ok(annotations) => annotations
                            .into_iter()
                            .map(|annotation_model| Annotation {
                                id: annotation_model.id,
                                title: annotation_model.title,
                                description: annotation_model.description,
                                created_at: annotation_model.created_at,
                                updated_at: annotation_model.updated_at,
                            })
                            .collect::<Vec<Annotation>>(),
                        Err(_) => Vec::new(),
                    },
                    created_at: person_model.created_at,
                    updated_at: person_model.updated_at,
                });
            }
            Ok(Json(persons))
        }
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
    State(state): State<ApplicationState>,
    Path(id): Path<u64>,
) -> Result<Json<Person>, (StatusCode, Json<GenericMessage>)> {
    match PersonModel::get(id, &state.database_connection).await {
        Ok(person_model) => {
            let cep = match Cep::try_from(person_model.cep.clone()) {
                Ok(cep) => cep,
                Err(_) => {
                    return Err((
                        StatusCode::NOT_FOUND,
                        Json(GenericMessage::new(404, "Person not found".to_string())),
                    ));
                }
            };
            let address = state.cep_service.get_address(cep).await;
            let person = Person {
                id: person_model.id,
                name: person_model.name,
                mothers_name: person_model.mothers_name,
                fathers_name: person_model.fathers_name,
                cep: person_model.cep,
                address: match address {
                    CepServiceResponse::CepFound(address) => Some(Address {
                        logradouro: address.logradouro.clone(),
                        complemento: address.complemento.clone(),
                        bairro: address.bairro.clone(),
                        localidade: address.localidade.clone(),
                        uf: address.uf.clone(),
                        ibge: address.ibge.clone(),
                        gia: address.gia.clone(),
                        ddd: address.ddd.clone(),
                        siafi: address.siafi.clone(),
                    }),
                    _ => None,
                },
                annotations: match AnnotationModel::list(
                    person_model.id,
                    &state.database_connection,
                )
                .await
                {
                    Ok(annotations) => annotations
                        .into_iter()
                        .map(|annotation_model| Annotation {
                            id: annotation_model.id,
                            title: annotation_model.title,
                            description: annotation_model.description,
                            created_at: annotation_model.created_at,
                            updated_at: annotation_model.updated_at,
                        })
                        .collect::<Vec<Annotation>>(),
                    Err(_) => Vec::new(),
                },
                created_at: person_model.created_at,
                updated_at: person_model.updated_at,
            };
            Ok(Json(person))
        }
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            Json(GenericMessage::new(404, "Person not found".to_string())),
        )),
    }
}

pub async fn create_person(
    State(state): State<ApplicationState>,
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
    State(state): State<ApplicationState>,
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
