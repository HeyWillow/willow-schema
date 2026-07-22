//! Semantic checks for the generated JSON Schema documents.

use pretty_assertions::assert_eq;
use serde_json::{Value, json};

const CONFIG_V1: &str = include_str!("../generated/json-schema/config-v1.schema.json");
const NVS_V1: &str = include_str!("../generated/json-schema/nvs-v1.schema.json");

fn document(source: &str) -> Value {
    serde_json::from_str(source).expect("generated JSON Schema should be valid JSON")
}

#[test]
fn documents_use_stable_ids_and_json_schema_draft() {
    let cases = [
        (
            document(CONFIG_V1),
            "urn:heywillow:schema:config:v1",
            "WillowConfigV1",
        ),
        (
            document(NVS_V1),
            "urn:heywillow:schema:nvs:v1",
            "WillowNvsConfigV1",
        ),
    ];

    for (schema, id, title) in cases {
        assert_eq!(
            schema["$schema"],
            "https://json-schema.org/draft/2020-12/schema"
        );
        assert_eq!(schema["$id"], id);
        assert_eq!(schema["title"], title);
    }
}

#[test]
fn config_schema_preserves_v1_wire_semantics() {
    let schema = document(CONFIG_V1);

    assert!(
        schema.get("required").is_none(),
        "all config fields are optional"
    );
    assert!(
        schema.get("additionalProperties").is_none(),
        "unknown config properties remain accepted"
    );
    assert_eq!(
        schema["properties"]["aec"]["type"],
        json!(["boolean", "null"])
    );
    assert_eq!(
        schema["properties"]["mqtt_port"]["type"],
        json!(["integer", "null"])
    );
    for property in [
        "display_timeout",
        "lvgl_lock_timeout",
        "stream_timeout",
        "vad_timeout",
    ] {
        assert_eq!(schema["properties"][property]["maximum"], u32::MAX);
    }
    assert_eq!(schema["$defs"]["VadMode"]["enum"], json!([0, 1, 2, 3, 4]));
}

#[test]
fn nvs_schema_preserves_required_fields_and_byte_constraints() {
    let schema = document(NVS_V1);

    assert_eq!(schema["required"], json!(["WAS", "WIFI"]));
    assert_eq!(schema["$defs"]["Was"]["required"], json!(["URL"]));
    assert_eq!(schema["$defs"]["Wifi"]["required"], json!(["PSK", "SSID"]));

    let was_url = &schema["$defs"]["WasUrl"];
    assert!(was_url.get("format").is_none());
    assert_eq!(was_url["pattern"], r"^wss?://[\s\S]*/ws$");
    assert_eq!(
        was_url["x-willow-url-validation"],
        json!({
            "parser": "whatwg",
            "pathnameSuffix": "/ws"
        })
    );

    let psk = &schema["$defs"]["WifiPsk"];
    assert_eq!(psk["minLength"], 8);
    assert_eq!(psk["maxLength"], 63);
    assert_eq!(psk["pattern"], r"^[\x20-\x7E]+$");
    assert_eq!(psk["x-willow-length-unit"], "bytes");

    let ssid = &schema["$defs"]["WifiSsid"];
    assert_eq!(ssid["minLength"], 1);
    assert_eq!(ssid["maxLength"], 32);
    assert_eq!(ssid["x-willow-max-utf8-bytes"], 32);
}
