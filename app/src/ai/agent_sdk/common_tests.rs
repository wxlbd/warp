use super::{default_text, parse_ambient_task_id};

#[test]
fn parse_ambient_task_id_accepts_valid_ids() {
    let prefix = default_text("agent_sdk.common.error.invalid_run_id");
    let task_id = parse_ambient_task_id("550e8400-e29b-41d4-a716-446655440000", &prefix).unwrap();

    assert_eq!(task_id.to_string(), "550e8400-e29b-41d4-a716-446655440000");
}

#[test]
fn parse_ambient_task_id_preserves_error_prefix() {
    let prefix = default_text("agent_sdk.common.error.invalid_run_id");
    let err = parse_ambient_task_id("not-a-run-id", &prefix).unwrap_err();

    assert!(err
        .to_string()
        .contains(&format!("{prefix} 'not-a-run-id'")));
}
