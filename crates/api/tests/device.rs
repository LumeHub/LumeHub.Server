mod common;

use std::sync::Arc;

use actix_web::http::StatusCode;
use actix_web::test::{self, TestRequest};
use actix_web::{App, web};
use application::SceneRuntime;
use common::{make_store, mock_runtime};
use serde_json::{Value, json};

macro_rules! svc {
    ($store:expr) => {{
        test::init_service(
            App::new()
                .app_data(web::Data::new($store))
                .app_data(web::Data::from(mock_runtime() as Arc<dyn SceneRuntime>))
                .configure(api::device::config),
        )
        .await
    }};
}

#[actix_web::test]
async fn get_state() {
    let svc = svc!(make_store().await);

    let resp = test::call_service(&svc, TestRequest::get().uri("/device/state").to_request()).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert!(body["on"].is_boolean());
    assert!(body["brightness"].is_number());
    assert!(body["color"].is_object());
}

#[actix_web::test]
async fn patch_state() {
    let svc = svc!(make_store().await);

    let resp = test::call_service(
        &svc,
        TestRequest::patch()
            .uri("/device/state")
            .set_json(json!({"on": false, "brightness": 100}))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert!(body["on"].is_boolean());
}
