use actix_web::http::StatusCode;
use actix_web::http::Uri;
use actix_web::HttpResponse;
use derive_new::new;
use lazy_static::lazy_static;
use serde::Serialize;
use std::env;
use std::sync::{Arc, Mutex, OnceLock};

lazy_static! {
    pub static ref BASE_URL: String = {
        let lock: OnceLock<String> = OnceLock::new();
        lock.get_or_init(|| env::var("HOST_URL").expect("Variable 'HOST_URL' not found"))
            .to_string()
    };
}

pub fn resource_not_found() -> HttpResponse {
    let error_response = GenericErrorResponse::new(
        (BASE_URL.clone() + "/resource_not_found")
            .parse::<Uri>()
            .unwrap()
            .to_string(),
        "Resource not found",
        (StatusCode::NOT_FOUND).as_u16(),
    );

    HttpResponse::NotFound().json(error_response)
}

pub fn rest_business_error(title: &str, code: &str) -> HttpResponse {
    let error_response = GenericErrorResponse::new(
        (BASE_URL.clone() + "/" + code)
            .parse::<Uri>()
            .unwrap()
            .to_string(),
        title,
        (StatusCode::UNPROCESSABLE_ENTITY).as_u16(),
    );
    HttpResponse::UnprocessableEntity().json(error_response)
}

pub fn created(location: Uri) -> HttpResponse {
    HttpResponse::Created()
        .append_header(("Location", location.to_string()))
        .finish()
}

pub fn no_content() -> HttpResponse {
    HttpResponse::new(StatusCode::NO_CONTENT)
}

#[derive(new, Debug, Serialize)]
pub struct GenericErrorResponse<'a> {
    #[serde(rename(serialize = "type"))]
    pub response_type: String,
    #[serde(rename(serialize = "title"))]
    pub response_title: &'a str,
    #[serde(rename(serialize = "status"))]
    pub response_status: u16,
    // #[serde(skip_serializing_if = "Vec::is_empty")]
    #[new(value = "vec![]")]
    pub invalid_params: Vec<ValidationError<'a>>,
}

#[derive(new, Debug, Serialize, Copy, Clone)]
pub struct ValidationError<'a> {
    pub message: &'a str,
}

pub fn to_invalid_param_bad_request(error_list: Arc<Mutex<Vec<ValidationError>>>) -> HttpResponse {
    let mut error_response = GenericErrorResponse::new(
        (BASE_URL.clone() + "/bad_request")
            .parse::<Uri>()
            .unwrap()
            .to_string(),
        "Bad request",
        (StatusCode::BAD_REQUEST).as_u16(),
    );

    error_list
        .lock()
        .unwrap()
        .iter()
        .for_each(|error| error_response.invalid_params.push(error.to_owned()));
    HttpResponse::BadRequest().json(error_response)
}
