mod common;

use std::sync::Arc;

use actix_web::http::StatusCode;
use actix_web::test::{self, TestRequest};
use actix_web::{App, web};
use application::SceneRuntime;
use common::{DefaultSpecRuntime, make_store, mock_runtime};
use domain::{TransitionCurve, TransitionSpec};
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
async fn put_scene_round_trips_opacity() {
    let svc = svc!(make_store().await);

    let resp = test::call_service(
        &svc,
        TestRequest::put()
            .uri("/scenes/active")
            .set_json(json!([
                {"effect_id": "builtin:rainbow", "zone_id": "all", "opacity": 0.25},
                {"effect_id": "builtin:aurora", "zone_id": "all"}
            ]))
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Vec<Value> = test::read_body_json(resp).await;
    assert_eq!(body[0]["opacity"].as_f64().unwrap(), 0.25);
    assert_eq!(body[1]["opacity"].as_f64().unwrap(), 1.0);
}

#[actix_web::test]
async fn added_layer_response_includes_default_opacity() {
    let svc = svc!(make_store().await);

    let resp = test::call_service(
        &svc,
        TestRequest::post()
            .uri("/scenes/active/layers")
            .set_json(json!({"effect_id": "builtin:rainbow", "zone_id": "all"}))
            .to_request(),
    )
    .await;
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["opacity"].as_f64().unwrap(), 1.0);
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

macro_rules! svc_with {
    ($runtime:expr) => {{
        test::init_service(
            App::new()
                .app_data(web::Data::new(make_store().await))
                .app_data(web::Data::from($runtime.clone() as Arc<dyn SceneRuntime>))
                .configure(api::active_scene::config)
                .configure(api::scenes::config),
        )
        .await
    }};
}

#[actix_web::test]
async fn add_layer_without_query_forwards_runtime_default_spec_to_reload() {
    let default = TransitionSpec::new(400, TransitionCurve::EaseInOut);
    let rec = Arc::new(DefaultSpecRuntime::new(default));
    let svc = svc_with!(rec);
    test::call_service(
        &svc,
        TestRequest::post()
            .uri("/scenes/active/layers")
            .set_json(json!({"effect_id": "builtin:rainbow", "zone_id": "all"}))
            .to_request(),
    )
    .await;
    assert_eq!(rec.last_reload.lock().unwrap().unwrap(), default);
}

#[actix_web::test]
async fn add_layer_with_fade_ms_forwards_overridden_spec_to_reload() {
    let rec = Arc::new(DefaultSpecRuntime::new(TransitionSpec::new(
        300,
        TransitionCurve::EaseInOut,
    )));
    let svc = svc_with!(rec);
    test::call_service(
        &svc,
        TestRequest::post()
            .uri("/scenes/active/layers?fade_ms=2500&curve=ease_in")
            .set_json(json!({"effect_id": "builtin:rainbow", "zone_id": "all"}))
            .to_request(),
    )
    .await;
    let spec = rec.last_reload.lock().unwrap().unwrap();
    assert_eq!(spec, TransitionSpec::new(2500, TransitionCurve::EaseIn));
}

#[actix_web::test]
async fn load_scene_forwards_fade_ms_to_reload() {
    let rec = Arc::new(DefaultSpecRuntime::new(TransitionSpec::INSTANT));
    let svc = svc_with!(rec);

    test::call_service(
        &svc,
        TestRequest::post()
            .uri("/scenes/active/layers")
            .set_json(json!({"effect_id": "builtin:rainbow", "zone_id": "all"}))
            .to_request(),
    )
    .await;
    let save_resp = test::call_service(
        &svc,
        TestRequest::post()
            .uri("/scenes/active/save")
            .set_json(json!({"name": "tv"}))
            .to_request(),
    )
    .await;
    let body: Value = test::read_body_json(save_resp).await;
    let scene_id = body["id"].as_str().unwrap().to_string();

    test::call_service(
        &svc,
        TestRequest::post()
            .uri(&format!(
                "/scenes/active/load/{scene_id}?fade_ms=10000&curve=ease_out"
            ))
            .to_request(),
    )
    .await;

    let spec = rec.last_reload.lock().unwrap().unwrap();
    assert_eq!(spec, TransitionSpec::new(10000, TransitionCurve::EaseOut));
}
