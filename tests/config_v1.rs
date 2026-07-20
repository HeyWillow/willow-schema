//! Compatibility tests for the version 1 configuration wire contract.

use pretty_assertions::assert_eq;
use serde::Serialize;
use serde_json::Value;
use willow_schema::config::v1::{
    AudioCodec, AudioResponseType, CommandEndpoint, Config, MqttAuthType, NtpConfig, RestAuthType,
    SpeechRecognitionMode, VadMode, WakeMode,
};

// Imported byte-for-byte from HeyWillow/willow-application-server,
// default_config.json at commit 94c9e47509d6086ba5396e0c3f28d417378032f4.
const WAS_PYTHON_PROVISIONING_V1: &str =
    include_str!("fixtures/config/was-python-provisioning-v1.json");

fn assert_wire_values<T: Serialize>(cases: &[(T, &str)]) {
    for (variant, expected) in cases {
        assert_eq!(
            serde_json::to_string(variant).expect("enum variant should serialize"),
            *expected
        );
    }
}

#[test]
fn audio_codec_wire_values_are_stable() {
    assert_wire_values(&[
        (AudioCodec::AmrWb, r#""AMR-WB""#),
        (AudioCodec::Pcm, r#""PCM""#),
        (AudioCodec::Wav, r#""WAV""#),
    ]);
}

#[test]
fn audio_response_type_wire_values_are_stable() {
    assert_wire_values(&[
        (AudioResponseType::Chimes, r#""Chimes""#),
        (AudioResponseType::None, r#""None""#),
        (AudioResponseType::Tts, r#""TTS""#),
    ]);
}

#[test]
fn command_endpoint_wire_values_are_stable() {
    assert_wire_values(&[
        (CommandEndpoint::HomeAssistant, r#""Home Assistant""#),
        (CommandEndpoint::Mqtt, r#""MQTT""#),
        (CommandEndpoint::OpenHab, r#""openHAB""#),
        (CommandEndpoint::Rest, r#""REST""#),
    ]);
}

#[test]
fn explicit_null_is_normalized_to_absent() {
    let config: Config = serde_json::from_str(
        r#"{
            "mqtt_port": null,
            "was_mode": null
        }"#,
    )
    .expect("null optional values should deserialize");

    assert_eq!(
        serde_json::to_value(config).expect("config should serialize"),
        serde_json::json!({})
    );
}

#[test]
fn legacy_wis_tts_fields_remain_accepted() {
    let config: Config = serde_json::from_str(
        r#"{
            "wis_tts_url": "https://legacy.example/tts",
            "wis_tts_url_v2": "https://v2.example/tts?text="
        }"#,
    )
    .expect("legacy WIS TTS fields should deserialize");

    assert_eq!(
        config.wis_tts_url.as_deref(),
        Some("https://legacy.example/tts")
    );
    assert_eq!(
        config.wis_tts_url_v2.as_deref(),
        Some("https://v2.example/tts?text=")
    );
}

#[test]
fn minimal_partial_firmware_config_deserializes() {
    let config: Config = serde_json::from_str(
        r#"{
            "aec": false,
            "display_timeout": 15
        }"#,
    )
    .expect("partial firmware config should deserialize");

    assert_eq!(config.aec, Some(false));
    assert_eq!(config.display_timeout, Some(15));
    assert_eq!(config.mqtt_port, None);
}

#[test]
fn mqtt_auth_type_wire_values_are_stable() {
    assert_wire_values(&[
        (MqttAuthType::None, r#""none""#),
        (MqttAuthType::UserPassword, r#""userpw""#),
    ]);
}

#[test]
fn ntp_config_wire_values_are_stable() {
    assert_wire_values(&[
        (NtpConfig::Dhcp, r#""DHCP""#),
        (NtpConfig::Host, r#""Host""#),
    ]);
}

#[test]
fn rest_auth_type_wire_values_are_stable() {
    assert_wire_values(&[
        (RestAuthType::Basic, r#""Basic""#),
        (RestAuthType::Header, r#""Header""#),
        (RestAuthType::None, r#""None""#),
    ]);
}

#[test]
fn speech_recognition_mode_wire_values_are_stable() {
    assert_wire_values(&[
        (SpeechRecognitionMode::Multinet, r#""Multinet""#),
        (SpeechRecognitionMode::Wis, r#""WIS""#),
    ]);
}

#[test]
fn unknown_properties_are_accepted_but_not_retained() {
    let config: Config = serde_json::from_str(
        r#"{
            "aec": true,
            "future_property": 42
        }"#,
    )
    .expect("unknown properties should be tolerated");

    assert_eq!(config.aec, Some(true));
    assert_eq!(
        serde_json::to_value(config).expect("config should serialize"),
        serde_json::json!({ "aec": true })
    );
}

#[test]
fn vad_mode_wire_values_are_stable() {
    assert_wire_values(&[
        (VadMode::Mode0, "0"),
        (VadMode::Mode1, "1"),
        (VadMode::Mode2, "2"),
        (VadMode::Mode3, "3"),
        (VadMode::Mode4, "4"),
    ]);
}

#[test]
fn wake_mode_wire_values_are_stable() {
    assert_wire_values(&[
        (WakeMode::OneChannel90, r#""1CH_90""#),
        (WakeMode::OneChannel95, r#""1CH_95""#),
        (WakeMode::ThreeChannel90, r#""3CH_90""#),
        (WakeMode::ThreeChannel95, r#""3CH_95""#),
        (WakeMode::TwoChannel90, r#""2CH_90""#),
        (WakeMode::TwoChannel95, r#""2CH_95""#),
    ]);
}

#[test]
fn was_python_provisioning_defaults_match_the_v1_golden_document() {
    let expected: Value = serde_json::from_str(WAS_PYTHON_PROVISIONING_V1)
        .expect("Python WAS provisioning fixture should be JSON");
    let defaults = Config::was_provisioning_defaults();
    let actual = serde_json::to_value(&defaults).expect("defaults should serialize");

    assert!(
        actual == expected,
        "Python WAS provisioning defaults differ from the v1 golden document"
    );
    let deployed = serde_json::from_value::<Config>(expected.clone())
        .expect("Python WAS provisioning fixture should deserialize");
    assert!(
        deployed == defaults,
        "Python WAS provisioning fixture differs from its Rust constructor"
    );
    assert!(expected["aec"].is_boolean());
    assert!(expected["mqtt_port"].is_number());
}

#[test]
fn wrongly_typed_and_unknown_known_values_are_rejected() {
    let cases = [
        ("boolean as string", r#"{"aec":"true"}"#),
        ("number as string", r#"{"mqtt_port":"1883"}"#),
        ("unknown codec", r#"{"audio_codec":"FLAC"}"#),
        ("unsupported VAD mode", r#"{"vad_mode":5}"#),
    ];

    for (name, input) in cases {
        assert!(
            serde_json::from_str::<Config>(input).is_err(),
            "{name} unexpectedly deserialized"
        );
    }
}
