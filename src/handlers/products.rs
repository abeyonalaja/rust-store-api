use crate::db_connection::{PgPool, PgPooledConnection};
use crate::handlers::LoggedUser;
use crate::models::product::{NewProduct, Product, ProductList};
use actix_web::{web, HttpRequest, HttpResponse};

fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
    pool.get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

// This is calling the list method on ProductList and
// serializing it to a json response
#[derive(Deserialize)]
pub struct ProductSearch {
    pub search: String,
}

pub fn index(
    _user: LoggedUser,
    pool: web::Data<PgPool>,
    product_search: web::Query<ProductSearch>,
) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    let search = &product_search.search;
    Ok(HttpResponse::Ok().json(ProductList::list(&pg_pool, search)))
}

pub fn create(
    new_product: web::Json<NewProduct>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    new_product
        .create(&pg_pool)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn show(id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Product::find(&id, &pg_pool)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn destroy(id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Product::destroy(&id, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn update(
    id: web::Path<i32>,
    new_product: web::Json<NewProduct>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Product::update(&id, &new_product, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
