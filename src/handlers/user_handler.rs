use actix_web::{ web, Scope, HttpResponse };

pub fn use_scope() -> Scope {
    web::scope("/user")
    .route("/encode-token/{id}", web::get().to(encode_token))
    .route("/decode-token", web::post().to(decode_token))
    .route("/protected", web::get().to(protected))
}

async fn encode_token() -> HttpResponse {
    HttpResponse::Ok().body("encode_token")
}

async fn decode_token() -> HttpResponse {
    HttpResponse::Ok().body("decode_token")
}

async fn protected() -> HttpResponse {
    HttpResponse::Ok().body("protected")
}
