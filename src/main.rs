use std::fs;

use gphoto2::{Camera, Context, Result};
use rouille::{router, Response};
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate rouille;

fn get_camera() -> Result<Camera> {
    return Context::new()?.autodetect_camera().wait();
}

#[derive(Serialize)]
struct JsonResponse {
    status: i32,
    message: String,
}

#[derive(Serialize)]
struct JsonEmpty {}

fn main() {
    let host = "localhost:8081";

    rouille::start_server(format!("{host}"), move |request| {
        router!(request,
            (GET) (/) => {
                rouille::Response::html(fs::read_to_string("./src/index.html").expect("Should have been able to read the file").replace("HOST", host))
            },
            (GET) (/capture_image) => {
                return match get_camera() {
                    Ok(c) => {
                        let _ = c.capture_image().wait();
                        Response::json(&JsonResponse{ status: 200, message: "Success".to_string() })
                    },
                    Err(err) => {
                        return Response::json(&JsonResponse{ status: 500, message: err.to_string() }).with_status_code(500);

                    }
                };
            },
            _ => rouille::Response::empty_404()
        )
    });
}
