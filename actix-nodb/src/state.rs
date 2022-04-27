//Application state is defined here
use std::sync::Mutex;

pub struct AppState {
    //Shared immutable state
    pub health_check_response: String,
    //Shared mutable state
    pub visit_count: Mutex<u32>,
}