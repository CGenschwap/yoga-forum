use crate::api;
use crate::auth::{self, JwtHandler, PasswordHandler};
use crate::error::YogaError;
use crate::rate_limiting::RateLimiting;
use crate::storage::StorageObj;
use crate::tracing::init_tracing;
use actix_cors::Cors;
use actix_web::{
    error, guard, http, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use anyhow::Result;
use std::path::PathBuf;

use tracing_actix_web::TracingLogger;

#[actix_web::main]
pub async fn server() -> Result<()> {
    init_tracing();

    let data_path = if let Ok(path) = std::env::var("YOGA_DATA") {
        PathBuf::from(&path)
    } else {
        PathBuf::from("./")
    };
    let storage = web::Data::new(StorageObj::new(&data_path).await?);

    let secret = std::env::var("YOGA_SECRET")
        .expect("Provide YOGA_SECRET for API signing and password encryption");
    let jwt_handler = web::Data::new(JwtHandler::new(secret.clone()));
    let password_handler = web::Data::new(PasswordHandler::new(secret));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://test.youonlygetanapi.com")
            .allowed_origin("https://youonlygetanapi.com")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .app_data(storage.clone())
            .app_data(jwt_handler.clone())
            .app_data(password_handler.clone())
            .app_data(web::JsonConfig::default().error_handler(json_error_handler))
            .wrap(cors)
            .wrap(RateLimiting::default())
            .wrap(middleware::Compress::default())
            .wrap(TracingLogger::default())
            .service(
                web::scope("/v1").service(
                    web::scope("/api")
                        // Users
                        .route("/users/login", web::post().to(auth::api::login))
                        .route("/users", web::post().to(auth::api::new_user))
                        .route("/users/find/{username}", web::get().to(api::find_user))
                        // Stories
                        .route("/stories", web::get().to(api::list_stories))
                        .route("/stories", web::post().to(api::new_story))
                        .route("/stories/{story_id}", web::get().to(api::get_story))
                        .route(
                            "/stories/{story_id}/comments",
                            web::get().to(api::list_comments),
                        )
                        .route(
                            "/stories/{story_id}/comments",
                            web::post().to(api::new_comment),
                        )
                        .route(
                            "/stories/{story_id}/comments/{comment_id}",
                            web::get().to(api::get_comment),
                        )
                        // MISC
                        .route("/ping", web::get().to(ping)),
                ), /*
                   .service(
                       web::scope("/static")
                           .route("/guide", web::get().to(guide))
                           .route("/swagger", web::get().to(swagger))
                           .route("/openapi", web::get().to(openapi)),
                   ),
                   */
            )
            /* This covers the "UI" portion of the site */
            // TODO: Use Caddy for this
            // TODO: Use fly.io for this
            //.service(web::scope("/index").route("", web::get().to(index)))
            /* Miscellaneous routes */
            .service(web::resource("/ping").route(web::get().to(ping)))
            .service(web::resource("/").route(web::get().to(redirect)))
            // TODO: We should route to a 404 page
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(default_handler),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}

async fn redirect() -> impl Responder {
    HttpResponse::PermanentRedirect()
        .append_header(("location", "/public/index.html"))
        .finish()
}

#[tracing::instrument]
async fn default_handler() -> impl Responder {
    HttpResponse::NotFound().json(serde_json::json!({
        "error": "NOT_FOUND",
        "message": "The path you requests was not found. Check for any typos"
    }))
}

#[tracing::instrument]
async fn ping() -> impl Responder {
    "Pong"
}

/*
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../../public/index.html"))
}

async fn guide() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../../public/guide.html"))
}

async fn swagger() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../../public/swagger.html"))
}

async fn openapi() -> impl Responder {
    let s = std::fs::read_to_string("./public/openapi.json").unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        //.body(include_str!("../../public/openapi.json"))
        .body(s)
}
*/

// TODO: How do we capture fields a user added in excess? We'd ideally
// like to capture those as errors
#[tracing::instrument]
fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;
    match &err {
        JsonPayloadError::ContentType => YogaError::InvalidMediaType(err.into()).into(),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            YogaError::InvalidRequest(err.into()).into()
        }
        _ => YogaError::InvalidRequest(err.into()).into(),
    }
}
