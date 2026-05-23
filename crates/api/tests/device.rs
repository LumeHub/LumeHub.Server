mod common;

use std::sync::Arc;

use actix_web::http::StatusCode;
use actix_web::test::{self, TestRequest};
use actix_web::{App, web};
use application::SceneRuntime;
use common::{DefaultSpecRuntime, make_store, mock_runtime};
use domain::{TransitionCurve, TransitionSpec};
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

fn recorder(default: TransitionSpec) -> Arc<DefaultSpecRuntime> {
    Arc::new(DefaultSpecRuntime::new(default))
}

async fn patch_with(uri: &str, body: Value, runtime: Arc<DefaultSpecRuntime>) {
    let svc = test::init_service(
        App::new()
            .app_data(web::Data::new(make_store().await))
            .app_data(web::Data::from(runtime as Arc<dyn SceneRuntime>))
            .configure(api::device::config),
    )
    .await;
    let resp = test::call_service(
        &svc,
        TestRequest::patch().uri(uri).set_json(body).to_request(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn patch_state_without_query_uses_runtime_default_spec() {
    let default = TransitionSpec::new(400, TransitionCurve::EaseInOut);
    let rec = recorder(default);
    patch_with("/device/state", json!({"on": true}), rec.clone()).await;
    assert_eq!(rec.last_on_off.lock().unwrap().unwrap().1, default);
}

#[actix_web::test]
async fn patch_state_with_fade_ms_overrides_duration() {
    let rec = recorder(TransitionSpec::new(400, TransitionCurve::EaseInOut));
    patch_with(
        "/device/state?fade_ms=1500",
        json!({"on": false}),
        rec.clone(),
    )
    .await;
    let (_, spec) = rec.last_on_off.lock().unwrap().unwrap();
    assert_eq!(spec.duration_ms, 1500);
    assert_eq!(spec.curve, TransitionCurve::EaseInOut);
}

#[actix_web::test]
async fn patch_state_with_curve_overrides_default_curve() {
    let rec = recorder(TransitionSpec::new(400, TransitionCurve::EaseInOut));
    patch_with(
        "/device/state?curve=exp",
        json!({"brightness": 80}),
        rec.clone(),
    )
    .await;
    let (value, spec) = rec.last_brightness.lock().unwrap().unwrap();
    assert_eq!(value, 80);
    assert_eq!(spec.curve, TransitionCurve::Exp);
}

#[actix_web::test]
async fn patch_state_fade_ms_zero_threads_instant_spec() {
    let rec = recorder(TransitionSpec::new(800, TransitionCurve::EaseInOut));
    patch_with(
        "/device/state?fade_ms=0",
        json!({"color": {"r": 1, "g": 2, "b": 3}}),
        rec.clone(),
    )
    .await;
    let (_, spec) = rec.last_color.lock().unwrap().unwrap();
    assert!(spec.is_instant());
}
