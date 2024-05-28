use crate::controllers::*;
use actix_web::web;
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/", web::get().to(index)) // GET request to "/"
            .route("/status", web::get().to(status)) // GET request to "/status"
    );
}