pub mod helpers;

use domain::{ParamControl, ParamDef, ParamValue, Rgb};

fn sample_params() -> Vec<ParamDef> {
    vec![
        ParamDef {
            name: "speed".to_string(),
            label: "Speed".to_string(),
            control: ParamControl::Slider {
                min: 0.5,
                max: 5.0,
                step: Some(0.5),
            },
            default: ParamValue::Number(1.0),
        },
        ParamDef {
            name: "color".to_string(),
            label: "Color".to_string(),
            control: ParamControl::Color,
            default: ParamValue::Color(Rgb { r: 255, g: 0, b: 0 }),
        },
        ParamDef {
            name: "reverse".to_string(),
            label: "Reverse".to_string(),
            control: ParamControl::Toggle,
            default: ParamValue::Bool(false),
        },
        ParamDef {
            name: "mode".to_string(),
            label: "Mode".to_string(),
            control: ParamControl::Select {
                options: vec!["linear".to_string(), "ease".to_string()],
            },
            default: ParamValue::Select("linear".to_string()),
        },
    ]
}

#[tokio::test]
async fn create_with_params_round_trips() {
    let store = helpers::in_memory_store().await;
    let params = sample_params();
    let effect = store
        .create_effect("wave", "let c = primary_color; c", &params)
        .await
        .unwrap();

    assert_eq!(effect.name, "wave");
    assert_eq!(effect.script, "let c = primary_color; c");
    assert_eq!(effect.params.len(), 4);

    let fetched = store.get_effect(&effect.id).await.unwrap();
    assert_eq!(fetched.params[0].name, "speed");
    assert!(matches!(
        fetched.params[0].control,
        ParamControl::Slider { min, max, .. } if (min - 0.5).abs() < f32::EPSILON && (max - 5.0).abs() < f32::EPSILON
    ));
    assert_eq!(fetched.params[0].default, ParamValue::Number(1.0));
    assert_eq!(
        fetched.params[1].default,
        ParamValue::Color(Rgb { r: 255, g: 0, b: 0 })
    );
    assert_eq!(fetched.params[2].default, ParamValue::Bool(false));
    assert_eq!(
        fetched.params[3].default,
        ParamValue::Select("linear".to_string())
    );
}

#[tokio::test]
async fn list_effects() {
    let store = helpers::in_memory_store().await;
    store
        .create_effect("a", "#{r:0,g:0,b:0}", &[])
        .await
        .unwrap();
    store
        .create_effect("b", "#{r:0,g:0,b:0}", &[])
        .await
        .unwrap();
    let effects = store.get_effects().await.unwrap();
    assert_eq!(effects.len(), 2);
}

#[tokio::test]
async fn update_effect() {
    let store = helpers::in_memory_store().await;
    let effect = store.create_effect("old", "code", &[]).await.unwrap();
    let updated = store
        .update_effect(&effect.id, "new", "new_code", &sample_params())
        .await
        .unwrap();
    assert_eq!(updated.name, "new");
    assert_eq!(updated.script, "new_code");
    assert_eq!(updated.params.len(), 4);
}

#[tokio::test]
async fn delete_effect() {
    let store = helpers::in_memory_store().await;
    let effect = store.create_effect("tmp", "code", &[]).await.unwrap();
    store.delete_effect(&effect.id).await.unwrap();
    let err = store.get_effect(&effect.id).await.unwrap_err();
    assert!(matches!(err, store::StoreError::NotFound));
}
