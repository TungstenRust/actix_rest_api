/** Contains handler functions that respond to HTTP requests
Health check handler using application state **/
use actix_web::{web, HttpResponse};
use super::state::AppState;
use super::models::Course;
use chrono::Utc;
/* Application state registered with the Actix web application is made available automatically
to all handler functions as an extractor object of type web::Data<T> where T is the type of the custom application state
 */
pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    //Data members of the Application state struct (AppState) can be directly accessed using standard dot notation
    let health_check_response = &app_state.health_check_response;
    /*Field representing shared mutable state (visit_count) has to be locked first before accessing,
    to prevent multiple threads from updating the value of the field simultaneously
    Construct response string to send back to browser client */
    let mut visit_count = app_state.visit_count.lock().unwrap();
    /* Update value of the field representing shared mutable state.
          Since the lock on this data has already been acquired,
          the value of the field can be updated safely.
          The lock on the data is automatically released when the handler function finishes execution. */
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}
//The handler function takes two parameters - data payload from HTTP request and application state.

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Received new course");
    let course_count_for_user = app_state
        .courses
        .lock()
        //Since courses collection is protected by Mutex,
        // I have to lock it first to access the data
        .unwrap()
        .clone()
        //Convert the course collection (stored within AppState) into an iterator,
        // so that I can iterate through each element in the collection for processing
        .into_iter()
        //Review each element in collection and filter only for the courses corresponding
        // to the tutor_id (received as part of the HTTP request)
        .filter(|course| course.tutor_id == new_course.tutor_id)
        //The filtered list of courses for the tutor is stored in a Vector
        .collect::<Vec<Course>>()
        //The number of elements in filtered list is computed.
        // This is used to generate the id for next course.
        .len();
    //Create a new course instance
    let new_course = Course {
        tutor_id: new_course.tutor_id,
        course_id: Some(course_count_for_user + 1),
        course_name: new_course.course_name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
    };
    //Add the new course instance to the course collection that is part of the application state (AppState)
    app_state.courses.lock().unwrap().push(new_course);
    //Send back an HTTP response to web client
    HttpResponse::Ok().json("Added course")
}
//The #[cfg(test)] annotation on tests module tells Rust to compile and run the tests only when cargo test command is run,
// and not for cargo build or cargo run
#[cfg(test)]
//Tests in Rust are written within the tests module.
mod tests {
    //Import all handler declarations from the parent module (which hosts the tests module)
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;
    //Normal rust tests are annotated with #[test].
    // But since this is an asynchronous test function,
    // I have to alert the async run-time of Actix-web to execute this async test function.
    #[actix_rt::test]
    async fn post_course_test() {
        //Construct a web::Json<T> object representing request data payload, i.e. new course data from tutor
        let course  = web::Json(Course {
            tutor_id: 1,
            course_name: "Hello, this is test course".into(),
            course_id: None,
            posted_time: None,
        });
        //Construct a web::Data<T> object representing application state
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });
        //Invoke handler function with application state and simulated request data payload
        let resp = new_course(course, app_state).await;
        //Verify if the HTTP status response code (returned from the handler) indicates success
        assert_eq!(resp.status(), StatusCode::OK);
    }
}