use std::fs;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, RwLock};

use axum::routing::{get, post};
use axum::Router;
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};

use mediamanager_api::routes;
use mediamanager_api::AppState;
use mediamanager_api::Config;

#[tokio::main]
async fn main() {
    let log_config = ConfigBuilder::new()
        .add_filter_allow_str(module_path!())
        .build();
    TermLogger::init(
        LevelFilter::Debug,
        log_config,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .expect("error");

    let config = fs::read_to_string("/etc/mediamanager.conf").unwrap();

    let config: Config = serde_yaml::from_str(&config).unwrap();

    let app_state = Arc::new(RwLock::new(AppState::new(config)));

    let addr = IpAddr::from(app_state.read().unwrap().config.network.address);
    let port = app_state.read().unwrap().config.network.port;

    println!("Listen on {}:{}", addr, port);
    let addr = SocketAddr::from((addr, port));

    let app = Router::new()
        .route("/", get(routes::root))
        .route("/jobs", get(routes::get_jobs))
        .route("/jobs", post(routes::create_job))
        .route("/clear", get(routes::clear_jobs))
        .with_state(app_state);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
