use crate::models::product::{NewProduct, ProductList};
use actix_web::{web, HttpRequest, HttpResponse};

// This is calling the list method on ProductList and
// serializing it to a json response
pub fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(ProductList::list())
}

pub fn create(new_product: web::Json<NewProduct>) -> Result<HttpResponse, HttpResponse> {
    new_product
        .create()
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
