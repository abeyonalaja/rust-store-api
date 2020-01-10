use crate::utils::jwt::create_token;
use actix_identity::Identity;
use actix_web::web;
use actix_web::HttpResponse;
use csrf_token::CsrfTokenGenerator;
use hex;

use crate::db_connection::PgPool;
use crate::errors::MyStoreError;
use crate::handlers::pg_pool_handler;
use crate::handlers::products::create;
use crate::models::user::AuthUser;
use diesel::pg::Pg;

// We get a new connection pool, then look up the user,
// If there is no user a Notfound error would raise otherwise
// this would just through an InternalServerError
pub fn login(
    auth_user: web::Json<AuthUser>,
    id: Identity,
    pool: web::Data<PgPool>,
    generator: web::Data<CsrfTokenGenerator>,
) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    let user = auth_user.login(&pg_pool).map_err(|e| match e {
        MyStoreError::DBError(diesel::result::Error::NotFound) => {
            HttpResponse::NotFound().json(e.to_string())
        }
        _ => HttpResponse::InternalServerError().json(e.to_string()),
    })?;

    // This is the jwt token we will have a csrf token for security
    let token = create_token(&user.email, &user.company)?;

    id.remember(token);

    let response = HttpResponse::Ok()
        .header("X-CSRF-TOKEN", hex::encode(generator.generate()))
        .json(user);
    Ok(response)
}

pub fn logout(id: Identity) -> Result<HttpResponse, HttpResponse> {
    id.forget();
    Ok(HttpResponse::Ok().into())
}
