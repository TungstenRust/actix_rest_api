//Contains the route definitions
use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}
pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(new_course))
            //Add new route for getting courses for a tutor (represented by the user_id variable)
            .route("/{user_id}", web::get().to(get_courses_for_tutor))
            //Add new route to get course details
            .route("/{user_id}/{course_id}", web::get().to(get_course_detail)),
    );
}