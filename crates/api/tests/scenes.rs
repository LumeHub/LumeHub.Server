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
                .configure(api::scenes::config),
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
            .uri("/scenes")
            .set_json(json!({"name": "Night"}))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["name"], "Night");
    let id = body["id"].as_str().unwrap().to_string();

    // get with empty layers
    let resp = test::call_service(
        &svc,
        TestRequest::get()
            .uri(&format!("/scenes/{id}"))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["layers"].as_array().unwrap().len(), 0);

    // list
    let resp = test::call_service(&svc, TestRequest::get().uri("/scenes").to_request()).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Vec<Value> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 1);

    // rename
    let resp = test::call_service(
        &svc,
        TestRequest::put()
            .uri(&format!("/scenes/{id}"))
            .set_json(json!({"name": "Day"}))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["name"], "Day");

    // delete
    let resp = test::call_service(
        &svc,
        TestRequest::delete()
            .uri(&format!("/scenes/{id}"))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = test::call_service(
        &svc,
        TestRequest::get()
            .uri(&format!("/scenes/{id}"))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
