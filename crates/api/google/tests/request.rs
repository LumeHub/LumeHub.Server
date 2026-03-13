use api_google::request::{ExecuteCommandType, Intent, SmartHomeRequest};

#[test]
fn intent_sync_deserializes() {
    let json = r#"{"requestId":"r1","inputs":[{"intent":"action.devices.SYNC"}]}"#;
    let req: SmartHomeRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.inputs[0].intent, Intent::Sync);
}

#[test]
fn intent_execute_deserializes() {
    let json = r#"{"requestId":"r2","inputs":[{"intent":"action.devices.EXECUTE","payload":{"commands":[]}}]}"#;
    let req: SmartHomeRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.inputs[0].intent, Intent::Execute);
}

#[test]
fn execute_command_type_on_off_deserializes() {
    let json = r#""action.devices.commands.OnOff""#;
    let cmd: ExecuteCommandType = serde_json::from_str(json).unwrap();
    assert_eq!(cmd, ExecuteCommandType::OnOff);
}

#[test]
fn command_request_get_params_extracts_value() {
    use api_google::request::{CommandRequest, DeviceRequest, ExecutionRequest};
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct OnOff {
        on: bool,
    }

    let cmd = CommandRequest {
        devices: vec![DeviceRequest {
            id: "led-strip".to_string(),
        }],
        execution: vec![ExecutionRequest {
            command: ExecuteCommandType::OnOff,
            params: serde_json::json!({"on": true}),
        }],
    };

    let params: OnOff = cmd.get_params().unwrap();
    assert!(params.on);
}

#[test]
fn command_request_get_params_errors_on_empty_execution() {
    use api_google::request::{CommandRequest, DeviceRequest};
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Dummy;

    let cmd = CommandRequest {
        devices: vec![DeviceRequest {
            id: "x".to_string(),
        }],
        execution: vec![],
    };

    assert!(cmd.get_params::<Dummy>().is_err());
}
