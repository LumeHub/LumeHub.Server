use effects::{load_builtins, validate_builtin_scripts};

#[test]
fn load_builtins_returns_nonempty_list() {
    let builtins = load_builtins();
    assert!(
        !builtins.is_empty(),
        "expected at least one built-in effect"
    );
}

#[test]
fn rainbow_builtin_is_present() {
    let builtins = load_builtins();
    assert!(
        builtins.iter().any(|b| b.slug == "rainbow"),
        "rainbow builtin should be present"
    );
}

#[test]
fn builtin_ids_have_builtin_prefix() {
    for b in load_builtins() {
        assert!(
            b.id().starts_with("builtin:"),
            "expected id to start with 'builtin:', got '{}'",
            b.id()
        );
    }
}

#[test]
fn validate_builtin_scripts_passes_for_all_stock_effects() {
    let errors = validate_builtin_scripts();
    assert!(
        errors.is_empty(),
        "stock builtin scripts have errors: {:?}",
        errors
    );
}

#[test]
fn all_builtins_have_nonempty_name_and_script() {
    for b in load_builtins() {
        assert!(!b.name.is_empty(), "builtin '{}' has empty name", b.slug);
        assert!(
            !b.script.is_empty(),
            "builtin '{}' has empty script",
            b.slug
        );
    }
}
