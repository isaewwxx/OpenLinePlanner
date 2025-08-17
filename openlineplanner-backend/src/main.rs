use std::path::PathBuf;
use std::sync::RwLock;
use std::time::Duration;

use actix_cors::Cors;
use actix_web::{
    middleware::{Logger, NormalizePath, TrailingSlash},
    web, App, HttpServer, HttpResponse,
};
use anyhow::Result;
use config::Config;
use error::OLPError;
use geo::Point;
use log::{info, warn};
use population::InhabitantsMap;
use serde::Deserialize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use validator::Validate;

mod coverage;
mod error;
mod geometry;
mod layers;
mod persistence;
mod population;
mod station;

use coverage::{CoverageMap, Method, Routing};
use layers::{LayerType, Layers};
use station::{OptimalStationResult, Station};

#[derive(Deserialize, Validate)]
struct StationInfoRequest {
    #[validate(length(min = 1, message = "At least one station is required"))]
    stations: Vec<Station>,
    #[validate(range(min = 100, max = 10000, message = "Separation distance must be between 100 and 10000 meters"))]
    separation_distance: Option<i32>,
    method: Option<Method>,
    routing: Option<Routing>,
}

#[derive(Deserialize, Validate)]
struct FindStationRequest {
    #[validate(length(min = 1, message = "At least one station is required"))]
    stations: Vec<Station>,
    #[validate(length(min = 2, message = "Route must have at least 2 points"))]
    route: Vec<Point>,
    method: Option<Method>,
    routing: Option<Routing>,
}

#[derive(Deserialize, Validate)]
struct Station {
    #[validate(length(min = 1, message = "Station ID is required"))]
    id: String,
    #[validate(range(min = -90.0, max = 90.0, message = "Latitude must be between -90 and 90"))]
    lat: f64,
    #[validate(range(min = -180.0, max = 180.0, message = "Longitude must be between -180 and 180"))]
    lng: f64,
    name: Option<String>,
}

async fn station_info(
    request: web::Json<StationInfoRequest>,
    layers: web::Data<RwLock<Layers>>,
) -> Result<HttpResponse, OLPError> {
    // Validate request
    request.validate().map_err(|e| OLPError::validation_error("Invalid request", Some(e)))?;

    let merged_layers = layers
        .read()
        .map_err(|e| OLPError::internal_error(format!("Failed to read layers: {}", e)))?
        .all_merged_by_type();

    let coverage_info: Vec<(LayerType, CoverageMap)> = merged_layers
        .iter()
        .map(|layer| {
            log::debug!("calculating for layer type: {}", layer.get_type());
            (
                layer.get_type().clone(),
                coverage::houses_for_stations(
                    &request.stations,
                    layer.get_centroids(),
                    &request.method.as_ref().unwrap_or(&Method::Relative),
                    &request.routing.as_ref().unwrap_or(&Routing::Osm),
                    layer.get_streets(),
                ),
            )
        })
        .collect();

    let coverage_slice: &[(LayerType, CoverageMap)] = &coverage_info;
    let inhabitants_map = population::InhabitantsMap::from(coverage_slice);

    Ok(HttpResponse::Ok().json(inhabitants_map))
}

async fn find_station(
    request: web::Json<FindStationRequest>,
    layers: web::Data<RwLock<Layers>>,
) -> Result<HttpResponse, OLPError> {
    // Validate request
    request.validate().map_err(|e| OLPError::validation_error("Invalid request", Some(e)))?;

    let layer = layers
        .read()
        .map_err(|e| OLPError::internal_error(format!("Failed to read layers: {}", e)))?
        .all_merged();

    let result = station::find_optimal_station(
        request.route.clone(),
        300f64,
        layer.get_centroids(),
        &request.stations,
        &request.method.as_ref().unwrap_or(&Method::Relative),
        &request.routing.as_ref().unwrap_or(&Routing::Osm),
        layer.get_streets(),
    );

    Ok(HttpResponse::Ok().json(result))
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn readiness(layers: web::Data<RwLock<Layers>>) -> HttpResponse {
    match layers.try_read() {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "ready",
            "layers_loaded": true
        })),
        Err(_) => HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "status": "not_ready",
            "layers_loaded": false
        }))
    }
}

fn setup_tracing() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "openlineplanner_backend=info,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setup structured logging
    setup_tracing().expect("failed to initialize tracing");

    info!("Starting OpenLinePlanner backend v{}", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            log::error!("Failed to load configuration: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    };

    let layers = load_layers(&config);
    let config = web::Data::new(config);

    info!("Data loading completed successfully");

    // Configure CORS
    let cors = configure_cors();

    HttpServer::new(move || {
        App::new()
            .wrap(cors.clone())
            .wrap(Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .app_data(layers.clone())
            .app_data(config.clone())
            .service(
                web::scope("/api/v1")
                    .route("/station-info", web::post().to(station_info))
                    .route("/coverage-info/{router}", web::post().to(coverage::coverage_info))
                    .route("/find-station", web::post().to(find_station))
                    .service(layers::layers())
                    .service(layers::osm())
            )
            .route("/health", web::get().to(health))
            .route("/ready", web::get().to(readiness))
            .default_service(web::route().to(|| async {
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": "NOT_FOUND",
                    "message": "Endpoint not found"
                }))
            }))
    })
    .bind(("0.0.0.0", 8080))?
    .workers(num_cpus::get())
    .backlog(1024)
    .keep_alive(Duration::from_secs(75))
    .client_timeout(Duration::from_secs(60))
    .client_disconnect_timeout(Duration::from_secs(30))
    .run()
    .await
}

fn load_config() -> Result<Config, OLPError> {
    let config = Config::builder()
        .set_default("cache.dir", "./cache/")?
        .set_default("data.dir", "./data/")?
        .set_default("server.workers", num_cpus::get())?
        .set_default("server.backlog", 1024)?
        .add_source(config::File::with_name("Config.toml").required(false))
        .add_source(config::Environment::with_prefix("OLP"))
        .build()?;

    Ok(config)
}

fn configure_cors() -> Cors {
    #[cfg(debug_assertions)]
    {
        Cors::permissive()
    }
    #[cfg(not(debug_assertions))]
    {
        Cors::default()
            .allowed_origin("https://openlineplanner.com")
            .allowed_origin("https://test.openlineplanner.com")
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600)
    }
}

fn load_layers(config: &Config) -> web::Data<RwLock<Layers>> {
    let mut path = PathBuf::from(config.get_string("cache.dir").unwrap_or_else(|_| "./cache/".to_string()));
    path.push("layers");
    
    match persistence::load_layers(&path) {
        Ok(layers) => {
            info!("Layers loaded successfully from {:?}", path);
            web::Data::new(RwLock::new(layers))
        }
        Err(e) => {
            warn!("Failed to load layers from {:?}: {}. Using empty layers.", path, e);
            web::Data::new(RwLock::new(Layers::default()))
        }
    }
}
