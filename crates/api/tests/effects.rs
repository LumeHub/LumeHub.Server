mod common;

use std::sync::Arc;

use actix_web::http::StatusCode;
use actix_web::test::{self, TestRequest};
use actix_web::{App, web};
use application::SceneRuntime;
use common::{make_store, mock_runtime};
use effects::load_builtins;
use serde_json::{Value, json};

macro_rules! svc {
    ($store:expr, $rt:expr) => {{
        test::init_service(
            App::new()
                .app_data(web::Data::new($store))
                .app_data(web::Data::from($rt as Arc<dyn SceneRuntime>))
                .app_data(web::Data::new(load_builtins()))
                .configure(api::effects::config),
        )
        .await
    }};
}

#[actix_web::test]
async fn list_includes_builtins() {
    let svc = svc!(make_store().await, mock_runtime());

    let resp = test::call_service(&svc, TestRequest::get().uri("/effects").to_request()).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Vec<Value> = test::read_body_json(resp).await;
    assert!(body.iter().any(|e| e["builtin"] == true));
}

#[actix_web::test]
async fn create_and_get() {
    let svc = svc!(make_store().await, mock_runtime());

    let resp = test::call_service(
        &svc,
        TestRequest::post()
            .uri("/effects")
            .set_json(json!({"name": "My Effect", "script": "rgb(255,0,0)", "params": []}))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["name"], "My Effect");
    assert_eq!(body["builtin"], false);
    let id = body["id"].as_str().unwrap().to_string();

    let resp = test::call_service(
        &svc,
        TestRequest::get()
            .uri(&format!("/effects/{id}"))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["id"], id);
}

#[actix_web::test]
async fn update_user_effect() {
    let svc = svc!(make_store().await, mock_runtime());

    let resp = test::call_service(
        &svc,
        TestRequest::post()
            .uri("/effects")
            .set_json(json!({"name": "Old", "script": "rgb(0,0,0)"}))
            .to_request(),
    )
    .await;
    let body: Value = test::read_body_json(resp).await;
    let id = body["id"].as_str().unwrap().to_string();

    let resp = test::call_service(
        &svc,
        TestRequest::put()
            .uri(&format!("/effects/{id}"))
            .set_json(json!({"name": "New", "script": "rgb(255,255,255)"}))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["name"], "New");
}

#[actix_web::test]
async fn delete_user_effect() {
    let svc = svc!(make_store().await, mock_runtime());

    let resp = test::call_service(
        &svc,
        TestRequest::post()
            .uri("/effects")
            .set_json(json!({"name": "Temp", "script": "rgb(0,0,0)"}))
            .to_request(),
    )
    .await;
    let body: Value = test::read_body_json(resp).await;
    let id = body["id"].as_str().unwrap().to_string();

    let del = test::call_service(
        &svc,
        TestRequest::delete()
            .uri(&format!("/effects/{id}"))
            .to_request(),
    )
    .await;
    assert_eq!(del.status(), StatusCode::NO_CONTENT);

    let get = test::call_service(
        &svc,
        TestRequest::get()
            .uri(&format!("/effects/{id}"))
            .to_request(),
    )
    .await;
    assert_eq!(get.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn builtin_is_immutable() {
    let svc = svc!(make_store().await, mock_runtime());

    let put = test::call_service(
        &svc,
        TestRequest::put()
            .uri("/effects/builtin:rainbow")
            .set_json(json!({"name": "Hacked", "script": "rgb(0,0,0)"}))
            .to_request(),
    )
    .await;
    assert_eq!(put.status(), StatusCode::FORBIDDEN);

    let del = test::call_service(
        &svc,
        TestRequest::delete()
            .uri("/effects/builtin:rainbow")
            .to_request(),
    )
    .await;
    assert_eq!(del.status(), StatusCode::FORBIDDEN);
}

#[actix_web::test]
async fn get_nonexistent_returns_404() {
    let svc = svc!(make_store().await, mock_runtime());

    let resp = test::call_service(
        &svc,
        TestRequest::get()
            .uri("/effects/does-not-exist")
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
