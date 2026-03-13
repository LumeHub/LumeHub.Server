use domain::Rgb;
use effects::preset::{EffectPreset, SignalColorDef, SignalDef};

#[test]
fn signal_color_def_converts_to_rgb() {
    let def = SignalColorDef {
        r: 10,
        g: 20,
        b: 30,
        speed: None,
    };
    let rgb = Rgb::from(&def);
    assert_eq!(rgb, Rgb::new(10, 20, 30));
}

#[test]
fn signal_def_deserializes_as_color_from_object() {
    let json = r#"{"r": 100, "g": 50, "b": 0}"#;
    let def: SignalDef = serde_json::from_str(json).unwrap();
    assert!(matches!(def, SignalDef::Color(_)));
}

#[test]
fn signal_def_deserializes_as_script_from_string() {
    let json = r#""from_hue(time * 0.1)""#;
    let def: SignalDef = serde_json::from_str(json).unwrap();
    assert!(matches!(def, SignalDef::Script(_)));
}

#[test]
fn effect_preset_deserializes_from_toml() {
    let toml = r#"
[[layers]]
effect = "script"
mode = "add"
code = "rgb(255, 0, 0)"
"#;
    let preset: EffectPreset = config::Config::builder()
        .add_source(config::File::from_str(toml, config::FileFormat::Toml))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap();
    assert_eq!(preset.layers.len(), 1);
    assert_eq!(preset.layers[0].effect, "script");
    assert_eq!(preset.layers[0].mode.as_deref(), Some("add"));
}
