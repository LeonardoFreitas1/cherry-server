mod handlers;
use std::io::Result;
use actix_web::{ App, HttpServer, middleware::Logger};
use handlers::user_handler::use_scope;

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
        .wrap(Logger::default())
        .service(use_scope())
    }).bind(("127.0.0.1", 9090))?
    .run()
        .await
}
