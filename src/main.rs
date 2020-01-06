pub mod db_connection;
pub mod handlers;
pub mod models;
pub mod schema;

//extern crate actix_web;
//use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate actix;
extern crate actix_web;
extern crate futures;
use actix_web::{web, App, HttpServer};

// index handler,
// returns a json response with an ok status
//fn index(_req: HttpRequest) -> HttpResponse {
//    HttpResponse::Ok().json("Hello world!")
//}

fn main() {
    //    HttpServer::new(|| App::new().service(web::resource("/").route(web::get().to_async(index))))
    //        .bind("127.0.0.1:8088")
    //        .unwrap()
    //        .run();

    let sys = actix::System::new("mystore");

    HttpServer::new(|| {
        App::new().service(
            web::resource("/products")
                .route(web::get().to_async(handlers::products::index))
                .route(web::post().to_async(handlers::products::create)),
        )
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .start();

    println!("Hello, world!");
    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
}
