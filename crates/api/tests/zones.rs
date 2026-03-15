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
                .configure(api::zones::config),
        )
        .await
    }};
}

#[actix_web::test]
async fn crud() {
    let svc = svc!(make_store().await);

    // create
    let resp = test::call_service(
        &svc,
        TestRequest::post()
            .uri("/zones")
            .set_json(json!({"name": "Full", "start_pixel": 0, "end_pixel": 100}))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["name"], "Full");
    assert_eq!(body["transition_length"], 0);
    let id = body["id"].as_str().unwrap().to_string();

    // get
    let resp = test::call_service(
        &svc,
        TestRequest::get().uri(&format!("/zones/{id}")).to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);

    // update
    let resp = test::call_service(
        &svc,
        TestRequest::put()
            .uri(&format!("/zones/{id}"))
            .set_json(json!({
                "name": "Renamed",
                "start_pixel": 0,
                "end_pixel": 50,
                "transition_length": 8
            }))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["name"], "Renamed");
    assert_eq!(body["transition_length"], 8);

    // list
    let resp = test::call_service(&svc, TestRequest::get().uri("/zones").to_request()).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Vec<Value> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 1);

    // delete
    let resp = test::call_service(
        &svc,
        TestRequest::delete()
            .uri(&format!("/zones/{id}"))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
}

#[actix_web::test]
async fn get_nonexistent_returns_404() {
    let svc = svc!(make_store().await);

    let resp = test::call_service(
        &svc,
        TestRequest::get().uri("/zones/no-such-zone").to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
