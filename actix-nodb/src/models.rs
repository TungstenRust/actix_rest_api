use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course {
    pub tutor_id: usize,
    pub course_id: Option<usize>,
    pub course_name: String,
    //NativeDateTime is a chrono data type for storing timestamp information
    pub posted_time: Option<NaiveDateTime>,
}
//Method to convert data from incoming HTTP request to Rust struct
impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        Course {
            tutor_id: course.tutor_id,
            course_id: course.course_id,
            course_name: course.course_name.clone(),
            posted_time: course.posted_time,
        }
    }
}