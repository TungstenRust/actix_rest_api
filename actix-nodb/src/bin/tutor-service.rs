//Building an Actix web server with application state
use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;
use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Initialize application state
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        //courses field initialized with a Mutex-protected empty vector
        courses: Mutex::new(vec![]),
    });
    // Define the web application
    let app = move || {
        // Register application state with the web application
        App::new()
            // Configure routes for the web application
            .app_data(shared_data.clone())
            .configure(general_routes)
            //Register the new course_routes group with application
            .configure(course_routes)
    };
    // Initialize Actix web server with the web application, listen on port 9000 and run the server
    HttpServer::new(app).bind("127.0.0.1:9000")?.run().await
}