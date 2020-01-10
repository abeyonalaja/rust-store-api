pub mod authentication;
pub mod products;
pub mod register;

use crate::db_connection::{PgPool, PgPooledConnection};
use actix_web::web;
use actix_web::HttpResponse;

// Because I'm using this function a lot,
// I'm including it in the mod file accessible to all handlers.
pub fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
    pool.get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

use crate::utils::jwt::{decode_token, SlimUser};
use actix_identity::Identity;
use actix_web::{dev, FromRequest, HttpRequest};
pub type LoggedUser = SlimUser;

use csrf_token::CsrfTokenGenerator;
use hex;

impl FromRequest for LoggedUser {
    type Error = HttpResponse;
    type Config = ();
    type Future = Result<Self, HttpResponse>;

    fn from_request(req: &HttpRequest, payload: &mut dev::Payload) -> Self::Future {
        let generator = req
            .app_data::<CsrfTokenGenerator>()
            .ok_or(HttpResponse::InternalServerError())?;

        let csrf_token = req
            .headers()
            .get("x-csrf-token")
            .ok_or(HttpResponse::Unauthorized())?;

        let decoded_token = hex::decode(&csrf_token)
            .map_err(|error| HttpResponse::InternalServerError().json(error.to_string()))?;

        generator
            .verify(&decoded_token)
            .map_err(|_| HttpResponse::Unauthorized())?;

        // We're using the CookieIdentityPolicy middleware
        // to handle cookies, with this implementation this
        // will validate the cookie according to the secret
        // provided in main function
        if let Some(identity) = Identity::from_request(req, payload)?.identity() {
            let user: SlimUser = decode_token(&identity)?;
            return Ok(user as LoggedUser);
        }
        Err(HttpResponse::Unauthorized().into())
    }
}
