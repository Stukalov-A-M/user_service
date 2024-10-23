mod api;

use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //TODO: Move to some non async config
    unsafe {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();
        App::new().service(api::user::test)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
