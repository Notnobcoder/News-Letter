use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web_prom::PrometheusMetricsBuilder;
use std::collections::HashMap;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8000;
    log::info!("Server running at port {}", port);
    let mut labels = HashMap::new();
    labels.insert("label1".to_string(), "value1".to_string());

    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(prometheus.clone())
            .service(routes::health)
            .service(routes::users)
            .service(routes::add_products)
            .service(routes::products)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

// ----------- MOVE TEST TO SEPERATE FILE -------------
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::{test, App};
//
//     #[actix_web::test]
//     async fn test_health_endpoint() {
//         let app = test::init_service(App::new().service(products)).await;
//
//         let req = test::TestRequest::get().uri("/health").to_request();
//         let resp = test::call_service(&app, req).await;
//
//         assert!(resp.status().is_success());
//
//         let body = test::read_body(resp).await;
//         assert_eq!(body, "OK");
//     }
//
//     #[actix_web::test]
//     async fn test_users_endpoint() {
//         let app = test::init_service(App::new().service(health).service(users)).await;
//
//         let req = test::TestRequest::get().uri("/users").to_request();
//         let resp = test::call_service(&app, req).await;
//
//         assert!(resp.status().is_success());
//     }
// }
