use actix_web::HttpResponse;

pub fn health_liveness() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn health_readiness() -> HttpResponse {
    HttpResponse::Ok().finish()
}
