mod common;

use std::sync::Arc;

use actix_web::http::StatusCode;
use actix_web::test::{self, TestRequest};
use actix_web::{App, web};
use application::SceneRuntime;
use common::{make_store, mock_runtime};
use serde_json::{Value, json};

// active_scene routes must come BEFORE scenes routes: /scenes/active must match
// before the wildcard /scenes/{id}, because actix-web uses FIFO route matching.
macro_rules! svc {
    ($store:expr) => {{
        test::init_service(
            App::new()
                .app_data(web::Data::new($store))
                .app_data(web::Data::from(mock_runtime() as Arc<dyn SceneRuntime>))
                .configure(api::active_scene::config)
                .configure(api::scenes::config),
        )
        .await
    }};
}

#[actix_web::test]
async fn starts_empty() {
    let svc = svc!(make_store().await);

    let resp =
        test::call_service(&svc, TestRequest::get().uri("/scenes/active").to_request()).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Vec<Value> = test::read_body_json(resp).await;
    assert!(body.is_empty());
}

#[actix_web::test]
async fn layer_lifecycle() {
    let svc = svc!(make_store().await);

    // add
    let resp = test::call_service(
        &svc,
        TestRequest::post()
            .uri("/scenes/active/layers")
            .set_json(json!({
                "effect_id": "builtin:rainbow",
                "zone_id": "all",
                "blend_mode": "override"
            }))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["effect_id"], "builtin:rainbow");
    assert_eq!(body["enabled"], true);
    let layer_id = body["id"].as_str().unwrap().to_string();

    // patch
    let resp = test::call_service(
        &svc,
        TestRequest::patch()
            .uri(&format!("/scenes/active/layers/{layer_id}"))
            .set_json(json!({"enabled": false}))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["enabled"], false);

    // still 1 layer in GET /scenes/active
    let resp =
        test::call_service(&svc, TestRequest::get().uri("/scenes/active").to_request()).await;
    let body: Vec<Value> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 1);

    // remove
    let resp = test::call_service(
        &svc,
        TestRequest::delete()
            .uri(&format!("/scenes/active/layers/{layer_id}"))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp =
        test::call_service(&svc, TestRequest::get().uri("/scenes/active").to_request()).await;
    let body: Vec<Value> = test::read_body_json(resp).await;
    assert!(body.is_empty());
}

#[actix_web::test]
async fn clear() {
    let svc = svc!(make_store().await);

    test::call_service(
        &svc,
        TestRequest::post()
            .uri("/scenes/active/layers")
            .set_json(
                json!({"effect_id": "builtin:aurora", "zone_id": "all", "blend_mode": "override"}),
            )
            .to_request(),
    )
    .await;

    let resp = test::call_service(
        &svc,
        TestRequest::delete().uri("/scenes/active").to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp =
        test::call_service(&svc, TestRequest::get().uri("/scenes/active").to_request()).await;
    let body: Vec<Value> = test::read_body_json(resp).await;
    assert!(body.is_empty());
}

#[actix_web::test]
async fn save_and_load() {
    let svc = svc!(make_store().await);

    test::call_service(
        &svc,
        TestRequest::post()
            .uri("/scenes/active/layers")
            .set_json(
                json!({"effect_id": "builtin:rainbow", "zone_id": "all", "blend_mode": "override"}),
            )
            .to_request(),
    )
    .await;

    // save
    let resp = test::call_service(
        &svc,
        TestRequest::post()
            .uri("/scenes/active/save")
            .set_json(json!({"name": "Saved Rainbow"}))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body: Value = test::read_body_json(resp).await;
    let scene_id = body["id"].as_str().unwrap().to_string();

    // clear, then load
    test::call_service(
        &svc,
        TestRequest::delete().uri("/scenes/active").to_request(),
    )
    .await;

    let resp = test::call_service(
        &svc,
        TestRequest::post()
            .uri(&format!("/scenes/active/load/{scene_id}"))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp =
        test::call_service(&svc, TestRequest::get().uri("/scenes/active").to_request()).await;
    let body: Vec<Value> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 1);
    assert_eq!(body[0]["effect_id"], "builtin:rainbow");
}

#[actix_web::test]
async fn load_nonexistent_returns_404() {
    let svc = svc!(make_store().await);

    let resp = test::call_service(
        &svc,
        TestRequest::post()
            .uri("/scenes/active/load/no-such-scene")
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
