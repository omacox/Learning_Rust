#[cfg(test)]
mod tests {
    use crate::handlers; // Import handlers from your crate root
    use actix_web::{http::StatusCode, test, web, App};

    #[actix_rt::test]
    async fn test_get_users() {
        let app = test::init_service(
            App::new().service(web::resource("/users").route(web::get().to(handlers::get_users))),
        )
        .await;

        let req = test::TestRequest::get().uri("/users").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
