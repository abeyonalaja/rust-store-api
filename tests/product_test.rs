#[macro_use]
extern crate dotenv_codegen;

mod common;

mod test {
    use actix_http::cookie::Cookie;
    use actix_http::httpmessage::HttpMessage;
    use actix_http::HttpService;
    use actix_http_test::{TestServer, TestServerRuntime};
    use actix_web::http::header;
    //    use actix_web::middleware::cors;
    //    use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService};
    use actix_web::{http, web, App};
    use chrono::Duration;
    use csrf_token::CsrfTokenGenerator;
    use http::header::HeaderValue;

    use crate::common::db_connection::establish_connection;
    use serde_json::json;
    use std::cell::{RefCell, RefMut};
    use std::str;
    use std::time::Duration as std_duration;

    use ::mystore_lib::models::product::{NewProduct, Product};

    #[test]
    fn test() {
        let mut srv = TestServer::new(|| {
            HttpService::new(
                App::new().data(establish_connection()).service(
                    web::resource("/products")
                        .route(web::get().to(::mystore_lib::handlers::products::index)),
                ),
            )
        });

        let request = srv.get("/products");
        let mut response = srv.block_on(request.send()).unwrap();
        assert!(response.status().is_success());

        assert_eq!(
            response.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "application/json"
        );

        let bytes = srv.block_on(response.body()).unwrap();
        let body = str::from_utf8(&bytes).unwrap();
        assert_eq!(body, "[]");
    }

    fn create_user() {
        use ::mystore_lib::models::user::{NewUser, User};
        use ::mystore_lib::schema::users;
        use chrono::Local;
        use diesel::RunQueryDsl;

        let connection = establish_connection();
        let pg_pool = connection.get().unwrap();

        diesel::delete(users::table).execute(&pg_pool).unwrap();

        diesel::insert_into(users::table)
            .values(NewUser {
                email: "jhon@doe.com".to_string(),
                company: "My own personal enterprise".to_string(),
                password: User::hash_password("12345678".to_string()).unwrap(),
                created_at: Local::now().naive_local(),
            })
            .get_result::<User>(&pg_pool)
            .unwrap();
    }

    fn login(mut srv: RefMut<TestServerRuntime>) -> (HeaderValue, Cookie) {
        let request = srv
            .post("/auth")
            .header(header::CONTENT_TYPE, "application/json")
            .timeout(std_duration::from_secs(600));
        let response = srv
            .block_on(request.send_body(r#"{"email":"jhon@doe.com","password":"12345678"}"#))
            .unwrap();
        let csrf_token = response.headers().get("x-csrf-token").unwrap();
        let cookies = response.cookies().unwrap();
        let cookie = cookies[0].clone().into_owned().value().to_string();

        let request_cookie = Cookie::build("mystorejwt", cookie)
            .domain("localhost")
            .path("/")
            .max_age(Duration::days(1).num_seconds())
            .secure(false)
            .http_only(false)
            .finish();
        (csrf_token.clone(), request_cookie.clone())
    }

}
