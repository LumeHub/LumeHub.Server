use mqtt::topics::Topics;

fn t() -> Topics {
    Topics::new("lumehub")
}

#[test]
fn device_topics() {
    let t = t();
    assert_eq!(t.device_state(), "lumehub/device/state");
    assert_eq!(t.device_set(), "lumehub/device/set");
}

#[test]
fn zone_topics() {
    let t = t();
    assert_eq!(t.zones(), "lumehub/zones");
    assert_eq!(t.zones_create(), "lumehub/zones/create");
    assert_eq!(t.zone("abc"), "lumehub/zones/abc");
    assert_eq!(t.zones_update_wildcard(), "lumehub/zones/+/update");
    assert_eq!(t.zones_delete_wildcard(), "lumehub/zones/+/delete");
}

#[test]
fn extract_id_success() {
    let t = t();
    assert_eq!(
        t.extract_id("lumehub/zones/abc123/update", "zones", "update"),
        Some("abc123")
    );
    assert_eq!(
        t.extract_id("lumehub/zones/xyz/delete", "zones", "delete"),
        Some("xyz")
    );
    assert_eq!(
        t.extract_id("lumehub/scenes/scene-1/load", "scenes", "load"),
        Some("scene-1")
    );
    assert_eq!(
        t.extract_id(
            "lumehub/active_layers/layer-id/update",
            "active_layers",
            "update"
        ),
        Some("layer-id")
    );
}

#[test]
fn extract_id_wrong_action() {
    let t = t();
    assert_eq!(
        t.extract_id("lumehub/zones/abc/update", "zones", "delete"),
        None
    );
}

#[test]
fn extract_id_wrong_resource() {
    let t = t();
    assert_eq!(
        t.extract_id("lumehub/scenes/abc/update", "zones", "update"),
        None
    );
}

#[test]
fn extract_id_rejects_nested_segments() {
    let t = t();
    // "a/b" as the ID segment should not match since it contains a slash
    assert_eq!(
        t.extract_id("lumehub/zones/a/b/update", "zones", "update"),
        None
    );
}

#[test]
fn extract_id_wrong_prefix() {
    let t = t();
    assert_eq!(
        t.extract_id("other/zones/abc/update", "zones", "update"),
        None
    );
}

#[test]
fn custom_prefix() {
    let t = Topics::new("myhome/lights");
    assert_eq!(t.device_state(), "myhome/lights/device/state");
    assert_eq!(t.zones_create(), "myhome/lights/zones/create");
    assert_eq!(
        t.extract_id("myhome/lights/zones/id1/update", "zones", "update"),
        Some("id1")
    );
}
