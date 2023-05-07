pub mod messages;
pub mod middlewares;
pub mod objects;
pub mod routers;
pub mod state;

use axum::{middleware, routing::IntoMakeService, Extension, Json, Router};
use cep_service::structs::service::CepService;
use clap::Parser;
use database::pool::connect;
use dotenv::dotenv;
use hyper::{
    header::{ACCEPT, AUTHORIZATION},
    server::conn::AddrIncoming,
    Method, Server, StatusCode,
};
use hyperlocal::{SocketIncoming, UnixServerExt};
use messages::GenericMessage;
use middlewares::authorization::auth;
use routers::{login, persons, users};
use std::{
    env,
    error::Error,
    net::SocketAddr,
    path::Path,
    sync::{Arc, RwLock},
};
use tokio::signal::ctrl_c;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum ServiceMode {
    Socket,
    Address,
}

#[derive(Parser)]
struct Opts {
    #[arg(short = 'm', long = "mode", value_enum, default_value = "address")]
    mode: ServiceMode,
}

fn socket_serve(rt: Router) -> Server<SocketIncoming, IntoMakeService<Router>> {
    let socket_addr = env::var("SOCKET_ADDR").expect("SOCKET_ADDR must be set.");
    let socket_file = Path::new(&socket_addr);
    let socket_folder = socket_file.parent().unwrap();
    match socket_folder.exists() {
        true => {
            if socket_folder.metadata().unwrap().permissions().readonly() {
                eprintln!("Socket folder is readonly.");
                std::process::exit(202);
            }
        }
        false => {
            eprintln!("Socket folder does not exist.");
            std::process::exit(202);
        }
    }
    match socket_file.exists() {
        true => {
            println!("Removing existing socket file.");
            std::fs::remove_file(socket_file).expect("Failed to remove socket file.");
        }
        false => println!("No existing socket file found."),
    }

    println!("Starting server on socket: {}", socket_addr);

    Server::bind_unix(socket_file)
        .expect("Failed to bind to socket.")
        .serve(rt.into_make_service())
}

fn address_serve(rt: Router) -> Server<AddrIncoming, IntoMakeService<Router>> {
    let address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS must be set.");
    let server_address: SocketAddr = address
        .parse::<SocketAddr>()
        .expect("Failed to parse server address.");

    println!("Starting server on address: {}", &address);

    Server::bind(&server_address).serve(rt.into_make_service())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let db_pool = Arc::new(connect().await.unwrap());
    let cep_service = Arc::new(RwLock::new(CepService::new()));
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::OPTIONS,
            Method::POST,
            Method::PUT,
            Method::DELETE,
        ])
        .allow_headers([AUTHORIZATION, ACCEPT]);
    let api = Router::new()
        .merge(login::get_router())
        .merge(users::get_router().layer(middleware::from_fn_with_state(db_pool.clone(), auth)))
        .merge(persons::get_router().layer(middleware::from_fn_with_state(db_pool.clone(), auth)));

    let app = Router::new()
        .nest("/api", api)
        .with_state(db_pool.clone())
        .layer(Extension(cep_service.clone()))
        .layer(cors)
        .fallback(deal_with_it);

    let opts: Opts = Opts::parse();
    let server_handle = tokio::spawn(async move {
        match opts.mode {
            ServiceMode::Socket => {
                let _ = socket_serve(app).await;
            }
            ServiceMode::Address => {
                let _ = address_serve(app).await;
            }
        }
    });

    ctrl_c().await.unwrap();
    server_handle.abort();
    Ok(())
}

async fn deal_with_it() -> (StatusCode, Json<GenericMessage>) {
    (
        StatusCode::NOT_FOUND,
        Json(messages::GenericMessage::new(404, "Not found".to_string())),
    )
}
