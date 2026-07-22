//! Compatibility tests for the version 1 NVS provisioning document.

use pretty_assertions::assert_eq;
use serde_json::Value;
use willow_schema::nvs::v1::{
    Config, WasUrl, WasUrlError, WifiPsk, WifiPskError, WifiSsid, WifiSsidError,
};

// Imported byte-for-byte as a migration oracle from
// HeyWillow/willow-application-server, default_nvs.json at commit
// f77e139bc4565b718c345e863ecd43ef7a06e81c.
const WAS_PYTHON_NVS_V1: &str = include_str!("fixtures/nvs/was-python-nvs-v1.json");

#[test]
fn all_known_properties_are_required() {
    let cases = [
        (
            "missing WAS",
            r#"{"WIFI":{"PSK":"password","SSID":"wifi"}}"#,
        ),
        ("missing WIFI", r#"{"WAS":{"URL":"wss://was.local/ws"}}"#),
        (
            "missing URL",
            r#"{"WAS":{},"WIFI":{"PSK":"password","SSID":"wifi"}}"#,
        ),
        (
            "missing PSK",
            r#"{"WAS":{"URL":"wss://was.local/ws"},"WIFI":{"SSID":"wifi"}}"#,
        ),
        (
            "missing SSID",
            r#"{"WAS":{"URL":"wss://was.local/ws"},"WIFI":{"PSK":"password"}}"#,
        ),
    ];

    for (name, input) in cases {
        assert!(
            serde_json::from_str::<Config>(input).is_err(),
            "{name} unexpectedly deserialized"
        );
    }
}

#[test]
fn unknown_properties_are_accepted_but_not_retained() {
    let input = serde_json::json!({
        "FUTURE": { "enabled": true },
        "WAS": {
            "FUTURE": 42,
            "URL": "wss://was.local/ws"
        },
        "WIFI": {
            "FUTURE": false,
            "PSK": "mypassword",
            "SSID": "myssid"
        }
    });
    let config: Config =
        serde_json::from_value(input).expect("unknown properties should be tolerated");

    assert_eq!(
        serde_json::to_value(config).expect("NVS document should serialize"),
        serde_json::json!({
            "WAS": { "URL": "wss://was.local/ws" },
            "WIFI": {
                "PSK": "mypassword",
                "SSID": "myssid"
            }
        })
    );
}

#[test]
fn was_python_nvs_document_is_the_v1_migration_oracle() {
    let expected: Value = serde_json::from_str(WAS_PYTHON_NVS_V1)
        .expect("Python WAS NVS fixture should contain JSON");
    let defaults = Config::was_provisioning_defaults();
    let actual = serde_json::to_value(&defaults).expect("defaults should serialize");

    assert_eq!(
        actual, expected,
        "Python WAS NVS defaults differ from the v1 migration oracle"
    );
    let document: Config = serde_json::from_value(expected.clone())
        .expect("Python WAS NVS fixture should deserialize");

    assert!(
        document == defaults,
        "Python WAS NVS fixture differs from its Rust constructor"
    );
    assert_eq!(document.was.url.as_str(), "wss://was.local/ws");
    assert_eq!(document.wifi.psk.as_str(), "mypassword");
    assert_eq!(document.wifi.ssid.as_str(), "myssid");
    assert_eq!(
        serde_json::to_value(document).expect("NVS document should serialize"),
        expected
    );
}

#[test]
fn was_url_matches_the_typescript_ui_contract() {
    for value in [
        "ws://was.local/ws",
        "wss://was.local/ws",
        "ws://was.local:8502/ws",
        "wss://[::1]:8503/prefix/ws",
        "wss://пример.рф/ws",
        "wss:////was.local/ws",
        "wss://user:secret@was.local/ws",
        "wss://was.local/ws?next=/ws",
        "wss://was.local/ws?token=secret/ws",
        "wss://was.local/ws#/ws",
    ] {
        assert!(
            WasUrl::try_from(String::from(value)).is_ok(),
            "{value} should be valid"
        );
    }

    let cases = [
        ("not a URL", WasUrlError::InvalidUrl),
        ("https://was.local/ws", WasUrlError::InvalidScheme),
        ("WS://was.local/ws", WasUrlError::InvalidScheme),
        ("wss://was.local/", WasUrlError::InvalidPath),
        ("wss://was.local/#/ws", WasUrlError::InvalidPath),
        ("wss://was.local?next=/ws", WasUrlError::InvalidPath),
        ("wss:///ws", WasUrlError::InvalidPath),
        ("wss://[v1.foo]/ws", WasUrlError::InvalidUrl),
    ];
    for (value, expected) in cases {
        assert_eq!(WasUrl::try_from(String::from(value)).err(), Some(expected));
        assert_eq!(
            expected.to_string(),
            match expected {
                WasUrlError::InvalidUrl => "URL is invalid",
                WasUrlError::InvalidScheme => "URL must start with ws:// or wss://",
                WasUrlError::InvalidPath => "URL must end with /ws",
            }
        );
    }
}

#[test]
fn wifi_psk_constraints_are_enforced() {
    let cases = [
        ("seven bytes", "1234567", WifiPskError::InvalidLength),
        (
            "64 bytes",
            "1234567890123456789012345678901234567890123456789012345678901234",
            WifiPskError::InvalidLength,
        ),
        (
            "control character",
            "password\n",
            WifiPskError::NonPrintableAscii,
        ),
        ("non-ASCII", "pässword", WifiPskError::NonPrintableAscii),
    ];

    for (name, input, expected) in cases {
        let result = WifiPsk::try_from(String::from(input));
        assert!(
            result == Err(expected),
            "{name} did not produce the expected error"
        );
    }

    assert!(WifiPsk::try_from(String::from("12345678")).is_ok());
    assert!(WifiPsk::try_from("a".repeat(63)).is_ok());
}

#[test]
fn wifi_ssid_constraints_are_enforced_in_bytes() {
    let cases = [
        ("empty", String::new()),
        ("33 ASCII bytes", "a".repeat(33)),
        ("33 UTF-8 bytes", format!("{}a", "🌿".repeat(8))),
    ];

    for (name, input) in cases {
        let result = WifiSsid::try_from(input);
        assert!(
            result == Err(WifiSsidError::InvalidLength),
            "{name} unexpectedly passed validation"
        );
    }

    assert!(WifiSsid::try_from("a".repeat(32)).is_ok());
    assert!(WifiSsid::try_from(String::from("Willow 🌿")).is_ok());
    assert!(WifiSsid::try_from("🌿".repeat(8)).is_ok());
}

#[test]
fn wrongly_typed_and_invalid_values_are_rejected() {
    let cases = [
        (
            "null WAS URL",
            r#"{"WAS":{"URL":null},"WIFI":{"PSK":"password","SSID":"wifi"}}"#,
        ),
        (
            "numeric PSK",
            r#"{"WAS":{"URL":"wss://was.local/ws"},"WIFI":{"PSK":12345678,"SSID":"wifi"}}"#,
        ),
        (
            "short PSK",
            r#"{"WAS":{"URL":"wss://was.local/ws"},"WIFI":{"PSK":"short","SSID":"wifi"}}"#,
        ),
        (
            "empty SSID",
            r#"{"WAS":{"URL":"wss://was.local/ws"},"WIFI":{"PSK":"password","SSID":""}}"#,
        ),
        (
            "fragment masquerading as WebSocket path",
            r#"{"WAS":{"URL":"wss://was.local/#/ws"},"WIFI":{"PSK":"password","SSID":"wifi"}}"#,
        ),
    ];

    for (name, input) in cases {
        assert!(
            serde_json::from_str::<Config>(input).is_err(),
            "{name} unexpectedly deserialized"
        );
    }
}
