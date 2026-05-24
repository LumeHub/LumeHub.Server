mod common;

use std::sync::Arc;

use actix_web::http::StatusCode;
use actix_web::test::{self, TestRequest};
use actix_web::{App, web};
use application::SceneRuntime;
use common::{DefaultSpecRuntime, make_store};
use domain::{TransitionCurve, TransitionSpec};
use serde_json::{Value, json};

macro_rules! svc {
    ($store:expr, $runtime:expr) => {{
        test::init_service(
            App::new()
                .app_data(web::Data::new($store))
                .app_data(web::Data::from($runtime.clone() as Arc<dyn SceneRuntime>))
                .configure(api::state_stack::config)
                .configure(api::active_scene::config),
        )
        .await
    }};
}

fn runtime() -> Arc<DefaultSpecRuntime> {
    Arc::new(DefaultSpecRuntime::new(TransitionSpec::INSTANT))
}

#[actix_web::test]
async fn empty_stack_returns_empty_list() {
    let svc = svc!(make_store().await, runtime());

    let resp = test::call_service(
        &svc,
        TestRequest::get().uri("/scenes/active/stack").to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Vec<Value> = test::read_body_json(resp).await;
    assert!(body.is_empty());
}

#[actix_web::test]
async fn pop_on_empty_stack_returns_404() {
    let svc = svc!(make_store().await, runtime());

    let resp = test::call_service(
        &svc,
        TestRequest::post().uri("/scenes/active/pop").to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn push_then_get_stack_lists_one_entry() {
    let svc = svc!(make_store().await, runtime());

    // add a layer so the snapshot has something to capture
    test::call_service(
        &svc,
        TestRequest::post()
            .uri("/scenes/active/layers")
            .set_json(json!({"effect_id": "builtin:rainbow", "zone_id": "all"}))
            .to_request(),
    )
    .await;

    let resp = test::call_service(
        &svc,
        TestRequest::post().uri("/scenes/active/push").to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = test::call_service(
        &svc,
        TestRequest::get().uri("/scenes/active/stack").to_request(),
    )
    .await;
    let body: Vec<Value> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 1);
    assert_eq!(body[0]["layer_count"], 1);
    assert!(body[0]["pushed_at"].as_i64().unwrap() > 0);
}

#[actix_web::test]
async fn push_pop_round_trips_active_layers() {
    let svc = svc!(make_store().await, runtime());

    // baseline state: one layer
    test::call_service(
        &svc,
        TestRequest::post()
            .uri("/scenes/active/layers")
            .set_json(json!({"effect_id": "builtin:rainbow", "zone_id": "all"}))
            .to_request(),
    )
    .await;

    // push baseline
    test::call_service(
        &svc,
        TestRequest::post().uri("/scenes/active/push").to_request(),
    )
    .await;

    // overwrite with TV scene
    test::call_service(
        &svc,
        TestRequest::put()
            .uri("/scenes/active")
            .set_json(json!([{"effect_id": "builtin:aurora", "zone_id": "all"}]))
            .to_request(),
    )
    .await;

    // pop restores the baseline
    let resp = test::call_service(
        &svc,
        TestRequest::post().uri("/scenes/active/pop").to_request(),
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
async fn pop_forwards_fade_ms_to_runtime() {
    let rec = runtime();
    let svc = svc!(make_store().await, rec);

    test::call_service(
        &svc,
        TestRequest::post()
            .uri("/scenes/active/layers")
            .set_json(json!({"effect_id": "builtin:rainbow", "zone_id": "all"}))
            .to_request(),
    )
    .await;
    test::call_service(
        &svc,
        TestRequest::post().uri("/scenes/active/push").to_request(),
    )
    .await;

    test::call_service(
        &svc,
        TestRequest::post()
            .uri("/scenes/active/pop?fade_ms=10000&curve=ease_out")
            .to_request(),
    )
    .await;

    let spec = rec.last_reload.lock().unwrap().unwrap();
    assert_eq!(spec, TransitionSpec::new(10000, TransitionCurve::EaseOut));
}

#[actix_web::test]
async fn clear_stack_drops_entries() {
    let svc = svc!(make_store().await, runtime());

    test::call_service(
        &svc,
        TestRequest::post()
            .uri("/scenes/active/layers")
            .set_json(json!({"effect_id": "builtin:rainbow", "zone_id": "all"}))
            .to_request(),
    )
    .await;
    test::call_service(
        &svc,
        TestRequest::post().uri("/scenes/active/push").to_request(),
    )
    .await;

    let resp = test::call_service(
        &svc,
        TestRequest::delete()
            .uri("/scenes/active/stack")
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = test::call_service(
        &svc,
        TestRequest::get().uri("/scenes/active/stack").to_request(),
    )
    .await;
    let body: Vec<Value> = test::read_body_json(resp).await;
    assert!(body.is_empty());
}

#[actix_web::test]
async fn stack_is_lifo() {
    let svc = svc!(make_store().await, runtime());

    test::call_service(
        &svc,
        TestRequest::post()
            .uri("/scenes/active/layers")
            .set_json(json!({"effect_id": "builtin:rainbow", "zone_id": "all"}))
            .to_request(),
    )
    .await;
    test::call_service(
        &svc,
        TestRequest::post().uri("/scenes/active/push").to_request(),
    )
    .await;

    test::call_service(
        &svc,
        TestRequest::put()
            .uri("/scenes/active")
            .set_json(json!([{"effect_id": "builtin:aurora", "zone_id": "all"}]))
            .to_request(),
    )
    .await;
    test::call_service(
        &svc,
        TestRequest::post().uri("/scenes/active/push").to_request(),
    )
    .await;

    // mutate again so we can prove pop restores `aurora` not `rainbow`
    test::call_service(
        &svc,
        TestRequest::put()
            .uri("/scenes/active")
            .set_json(json!([{"effect_id": "builtin:lava", "zone_id": "all"}]))
            .to_request(),
    )
    .await;

    test::call_service(
        &svc,
        TestRequest::post().uri("/scenes/active/pop").to_request(),
    )
    .await;
    let resp =
        test::call_service(&svc, TestRequest::get().uri("/scenes/active").to_request()).await;
    let body: Vec<Value> = test::read_body_json(resp).await;
    assert_eq!(body[0]["effect_id"], "builtin:aurora");
}
