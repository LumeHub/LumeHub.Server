use effects::config::EffectsConfig;
use std::path::PathBuf;

#[test]
fn embedded_presets_are_loaded() {
    let dir = PathBuf::from("/nonexistent");
    let config = EffectsConfig::from_dir(&dir);
    assert!(
        !config.presets.is_empty(),
        "embedded presets should be non-empty"
    );
}

#[test]
fn known_stock_preset_is_present() {
    let config = EffectsConfig::from_dir(&PathBuf::from("/nonexistent"));
    assert!(
        config.presets.contains_key("rainbow"),
        "rainbow preset should be present"
    );
}

#[test]
fn fs_presets_override_embedded_with_same_name() {
    let dir = tempfile::tempdir().unwrap();
    let effects_dir = dir.path().join("effects");
    std::fs::create_dir_all(&effects_dir).unwrap();
    std::fs::write(effects_dir.join("rainbow.rhai"), "rgb(0, 0, 0)").unwrap();

    let config = EffectsConfig::from_dir(dir.path());
    let preset = config.presets.get("rainbow").unwrap();
    // The fs preset replaced the embedded one (single script layer, code matches)
    assert_eq!(preset.layers.len(), 1);
}
