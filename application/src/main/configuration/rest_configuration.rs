use crate::main::configuration::use_case_configuration::{
    ADD_MEAL_TO_MEANU_USE_CASE, GET_MEAL_BY_ID_USE_CASE, GET_MENU_USE_CASE,
};
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use rest::main::endpoint_url::{MENU_ADD_TO_MENU, MENU_GET_ALL, MENU_GET_BY_ID};
use rest::main::menu::{
    add_meal_to_menu_endpoint, get_health_status, get_meal_by_id_endpoint, get_menu_endpoint,
};
use usecase::main::menu::scenario::add_meal_to_menu_use_case::AddMealToMenuUseCase;
use usecase::main::menu::scenario::get_meal_by_id_use_case::GetMealByIdUseCase;
use usecase::main::menu::scenario::get_menu_use_case::GetMenuUseCase;

#[actix_web::main]
pub async fn start_web_backend() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(ADD_MEAL_TO_MEANU_USE_CASE.clone())
            .app_data(GET_MEAL_BY_ID_USE_CASE.clone())
            .app_data(GET_MENU_USE_CASE.clone())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .route(
                MENU_ADD_TO_MENU,
                web::post().to(add_meal_to_menu_endpoint::execute::<AddMealToMenuUseCase>),
            )
            .route(
                MENU_GET_BY_ID,
                web::get().to(get_meal_by_id_endpoint::execute::<GetMealByIdUseCase>),
            )
            .route(
                MENU_GET_ALL,
                web::get().to(get_menu_endpoint::execute::<GetMenuUseCase>),
            )
            .route("/health", web::get().to(get_health_status::execute))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
