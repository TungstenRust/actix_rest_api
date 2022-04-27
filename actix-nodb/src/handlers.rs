/** Contains handler functions that respond to HTTP requests
Health check handler using application state **/

use super::state::AppState;
use actix_web::{web, HttpResponse};
/** Application state registered with the Actix web application is made available automatically
to all handler functions as an extractor object of type web::Data<T> where T is the type of the custom application state
 **/
pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    //Data members of the Application state struct (AppState) can be directly accessed using standard dot notation
    let health_check_response = &app_state.health_check_response;
    /** Field representing shared mutable state (visit_count) has to be locked first before accessing,
          to prevent multiple threads from updating the value of the field simultaneously **/
    //Construct response string to send back to browser client
    let mut visit_count = app_state.visit_count.lock().unwrap();
    /** Update value of the field representing shared mutable state.
          Since the lock on this data has already been acquired,
          the value of the field can be updated safely.
          The lock on the data is automatically released when the handler function finishes execution. **/
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}