//! Checks for the generated WAS provisioning default documents.

use pretty_assertions::assert_eq;
use serde::Serialize;
use serde_json::Value;
use willow_schema::{config, nvs};

const CONFIG_V1: &str = include_str!("../generated/defaults/config-v1.json");
const NVS_V1: &str = include_str!("../generated/defaults/nvs-v1.json");

fn assert_generated_defaults<T: Serialize>(source: &str, defaults: &T) {
    let generated: Value =
        serde_json::from_str(source).expect("generated defaults should contain JSON");
    let expected = serde_json::to_value(defaults).expect("typed defaults should serialize");

    assert_eq!(generated, expected);
}

#[test]
fn config_v1_matches_typed_was_provisioning_defaults() {
    assert_generated_defaults(CONFIG_V1, &config::v1::Config::was_provisioning_defaults());
}

#[test]
fn nvs_v1_matches_typed_was_provisioning_defaults() {
    assert_generated_defaults(NVS_V1, &nvs::v1::Config::was_provisioning_defaults());
}
