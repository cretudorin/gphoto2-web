use crate::responses::JsonResponse;
use actix_files as fs;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use gphoto2::{list::CameraDescriptor, Camera, Context, Result};
pub mod responses;

#[get("/cameras")]
async fn get_cameras() -> impl Responder {
    let cameras = Context::new().unwrap().list_cameras();
    HttpResponse::Ok().json(&JsonResponse {
        status: 200,
        message: "foo".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/", "./src/static").index_file("index.html"))
            .service(get_cameras)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
