use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

// --- TYPES (MOVE INTO SEPERARE FILE) ----
#[derive(Serialize, Deserialize)]
struct UserPayload {
    name: String,
    age: u32,
}

#[derive(Serialize, Deserialize)]
struct ProductPayload {
    name: String,
    age: u32,
}

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/users")]
pub async fn users() -> impl Responder {
    let user = UserPayload {
        name: "John Doe".to_string(),
        age: 43,
    };

    HttpResponse::Ok().json(user)
}

#[get("/products")]
pub async fn products() -> impl Responder {
    let product = ProductPayload {
        name: "testing".to_string(),
        age: 32,
    };

    HttpResponse::Ok().json(product)
}

#[post("/products")]
pub async fn add_products(data: web::Json<ProductPayload>) -> impl Responder {
    HttpResponse::Ok().json(data)
}
