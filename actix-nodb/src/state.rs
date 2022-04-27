//Application state is defined here
use super::models::Course;
use std::sync::Mutex;


pub struct AppState {
    //Shared immutable state
    pub health_check_response: String,
    //Shared mutable state
    pub visit_count: Mutex<u32>,
    //Courses are stored in application state as a Vec collection, protected by a Mutex.
    pub courses: Mutex<Vec<Course>>
}
