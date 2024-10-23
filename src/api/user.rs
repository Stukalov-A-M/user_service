use actix_web::{get, web::Json};

#[get("/users/test")]
async fn test() -> Json<String> {
    Json("Hello, world!".to_string())
}
