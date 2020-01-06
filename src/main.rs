extern crate actix_web;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

// index handler,
// returns a json response with an ok status
fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json("Hello world!")
}

fn main() {
    HttpServer::new(|| App::new().service(web::resource("/").route(web::get().to_async(index))))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
    //    println!("Hello, world!");
}
