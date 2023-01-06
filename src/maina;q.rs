mod handlers;
use std::io::Result;
use actix_web::{ App, HttpServer, middleware::Logger};
use handlers::user_handler::use_scope;
use sqlx::{postgres::PgPoolOptions};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("expected database url");
    env_logger::init();

    let _db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await;
    
    HttpServer::new(|| {
        App::new()
        .wrap(Logger::default())
        .service(use_scope())
    }).bind(("127.0.0.1", 9090))?
    .run().await
}
