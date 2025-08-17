pub mod error_tests;
pub mod station_tests;
pub mod geometry_tests;

#[cfg(test)]
mod common {
    use super::*;
    use crate::error::OLPError;
    use actix_web::test;
    use serde_json::json;

    pub async fn create_test_app() -> impl actix_web::dev::Service<
        actix_web::http::Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    > {
        use crate::main;
        test::init_service(
            actix_web::App::new()
                .configure(|cfg| {
                    // Configure test routes here
                })
        )
        .await
    }

    pub fn create_test_station() -> crate::station::Station {
        crate::station::Station {
            id: "test-station-1".to_string(),
            lat: 48.2082,
            lng: 16.3738,
            name: Some("Test Station".to_string()),
        }
    }

    pub fn create_test_stations() -> Vec<crate::station::Station> {
        vec![
            create_test_station(),
            crate::station::Station {
                id: "test-station-2".to_string(),
                lat: 48.2100,
                lng: 16.3750,
                name: Some("Test Station 2".to_string()),
            },
        ]
    }
}