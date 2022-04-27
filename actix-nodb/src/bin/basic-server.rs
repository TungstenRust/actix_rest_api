// Module imports
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// Configure route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

//Configure handler
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello. Actix-REST is alive and kicking")
}

// Initiate and run the HTTP server
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Construct app and configure routes
    let app = move || App::new().configure(general_routes);
    // Start HTTP server
    HttpServer::new(app).bind("127.0.0.1:9000")?.run().await
}