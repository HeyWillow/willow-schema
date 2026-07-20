//! The existing, flat Willow configuration wire format.

use alloc::string::String;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Audio encoding sent to Willow Inference Server.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AudioCodec {
    /// Adaptive Multi-Rate Wideband audio.
    #[serde(rename = "AMR-WB")]
    AmrWb,
    /// Raw PCM audio.
    #[serde(rename = "PCM")]
    Pcm,
    /// WAV-wrapped PCM audio.
    #[serde(rename = "WAV")]
    Wav,
}

/// Audible response behavior after command execution.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AudioResponseType {
    /// Play response chimes.
    Chimes,
    /// Produce no audible response.
    #[serde(rename = "None")]
    None,
    /// Speak the response using text-to-speech.
    #[serde(rename = "TTS")]
    Tts,
}

/// Destination used to execute recognized commands.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CommandEndpoint {
    /// Home Assistant.
    #[serde(rename = "Home Assistant")]
    HomeAssistant,
    /// An MQTT broker.
    #[serde(rename = "MQTT")]
    Mqtt,
    /// openHAB.
    #[serde(rename = "openHAB")]
    OpenHab,
    /// A generic REST endpoint.
    #[serde(rename = "REST")]
    Rest,
}

/// Authentication mode used by the MQTT command endpoint.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum MqttAuthType {
    /// No authentication.
    #[serde(rename = "none")]
    None,
    /// User name and password authentication.
    #[serde(rename = "userpw")]
    UserPassword,
}

/// Source of the NTP server configuration.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum NtpConfig {
    /// Obtain the NTP host through DHCP.
    #[serde(rename = "DHCP")]
    Dhcp,
    /// Use the explicitly configured NTP host.
    Host,
}

/// Authentication mode used by the generic REST command endpoint.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum RestAuthType {
    /// HTTP Basic authentication.
    Basic,
    /// Authentication through a custom request header.
    Header,
    /// No authentication.
    #[serde(rename = "None")]
    None,
}

/// Speech-recognition implementation selected by the firmware.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SpeechRecognitionMode {
    /// On-device Multinet recognition.
    Multinet,
    /// Willow Inference Server recognition.
    #[serde(rename = "WIS")]
    Wis,
}

/// ESP-SR voice-activity detector aggressiveness.
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum VadMode {
    /// Mode 0.
    Mode0 = 0,
    /// Mode 1.
    Mode1 = 1,
    /// Mode 2.
    Mode2 = 2,
    /// Mode 3.
    Mode3 = 3,
    /// Mode 4.
    Mode4 = 4,
}

/// `WakeNet` recognition channel and sensitivity mode.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum WakeMode {
    /// One-channel recognition at the 90% sensitivity setting.
    #[serde(rename = "1CH_90")]
    OneChannel90,
    /// One-channel recognition at the 95% sensitivity setting.
    #[serde(rename = "1CH_95")]
    OneChannel95,
    /// Three-channel recognition at the 90% sensitivity setting.
    #[serde(rename = "3CH_90")]
    ThreeChannel90,
    /// Three-channel recognition at the 95% sensitivity setting.
    #[serde(rename = "3CH_95")]
    ThreeChannel95,
    /// Two-channel recognition at the 90% sensitivity setting.
    #[serde(rename = "2CH_90")]
    TwoChannel90,
    /// Two-channel recognition at the 95% sensitivity setting.
    #[serde(rename = "2CH_95")]
    TwoChannel95,
}

/// The existing, flat Willow configuration v1 JSON document.
///
/// Every known property is optional because deployed firmware accepts partial
/// documents. A missing known property and a known property explicitly set to
/// JSON `null` both deserialize as `None`. `None` values are omitted during
/// serialization, so an explicit `null` is normalized to an absent property by
/// a typed deserialize/serialize cycle.
///
/// Unknown object properties are accepted during deserialization but are not
/// retained. This typed representation therefore does not promise lossless
/// round-tripping.
///
/// `v1` is the Rust module version. No version property is added to the wire
/// document.
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    /// Enables acoustic echo cancellation on the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aec: Option<bool>,
    /// Selects the audio encoding used for WIS streaming.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_codec: Option<AudioCodec>,
    /// Selects the audible command-response behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_response_type: Option<AudioResponseType>,
    /// Enables blind source separation on the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bss: Option<bool>,
    /// Selects the service that executes recognized commands.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_endpoint: Option<CommandEndpoint>,
    /// Sets the display timeout in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_timeout: Option<u32>,
    /// Sets the Home Assistant host name or address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hass_host: Option<String>,
    /// Sets the Home Assistant TCP port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hass_port: Option<u16>,
    /// Enables TLS for Home Assistant connections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hass_tls: Option<bool>,
    /// Sets the Home Assistant access token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hass_token: Option<String>,
    /// Sets the LCD backlight PWM level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lcd_brightness: Option<u16>,
    /// Sets the LVGL lock timeout in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lvgl_lock_timeout: Option<u32>,
    /// Sets the microphone gain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mic_gain: Option<u8>,
    /// Selects the MQTT authentication mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mqtt_auth_type: Option<MqttAuthType>,
    /// Sets the MQTT broker host name or address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mqtt_host: Option<String>,
    /// Sets the MQTT password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mqtt_password: Option<String>,
    /// Sets the MQTT broker TCP port as a JSON number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mqtt_port: Option<u16>,
    /// Enables TLS for MQTT connections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mqtt_tls: Option<bool>,
    /// Sets the MQTT topic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mqtt_topic: Option<String>,
    /// Sets the MQTT user name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mqtt_username: Option<String>,
    /// Enables Willow One Wake arbitration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiwake: Option<bool>,
    /// Selects the NTP configuration source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ntp_config: Option<NtpConfig>,
    /// Sets the explicitly configured NTP host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ntp_host: Option<String>,
    /// Sets the openHAB access token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openhab_token: Option<String>,
    /// Sets the openHAB base URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openhab_url: Option<String>,
    /// Sets the recorder ring-buffer size in KiB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_buffer: Option<u16>,
    /// Sets the header value used for REST Header authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_auth_header: Option<String>,
    /// Sets the password used for REST Basic authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_auth_pass: Option<String>,
    /// Selects the REST authentication mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_auth_type: Option<RestAuthType>,
    /// Sets the user name used for REST Basic authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_auth_user: Option<String>,
    /// Sets the generic REST command endpoint URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_url: Option<String>,
    /// Includes prerelease firmware builds in upgrade listings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_prereleases: Option<bool>,
    /// Sets the speaker volume percentage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_volume: Option<u8>,
    /// Selects the speech-recognition implementation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_rec_mode: Option<SpeechRecognitionMode>,
    /// Sets the speech stream or session timeout in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_timeout: Option<u32>,
    /// Sets the POSIX timezone string used by the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Sets the human-readable IANA timezone selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone_name: Option<String>,
    /// Selects the voice-activity detector aggressiveness.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vad_mode: Option<VadMode>,
    /// Sets the silence duration that ends speech, in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vad_timeout: Option<u32>,
    /// Enables the wake-confirmation tone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wake_confirmation: Option<bool>,
    /// Selects the `WakeNet` channel and sensitivity mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wake_mode: Option<WakeMode>,
    /// Sets the wake-word or model identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wake_word: Option<String>,
    /// Routes command execution through WAS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_mode: Option<bool>,
    /// Sets the legacy base WIS TTS URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wis_tts_url: Option<String>,
    /// Sets the WIS TTS URL template containing the text query parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wis_tts_url_v2: Option<String>,
    /// Sets the WIS speech-recognition URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wis_url: Option<String>,
}

impl Config {
    /// Constructs the deployed user-facing provisioning document produced by
    /// the Python Willow Application Server.
    #[must_use]
    pub fn was_provisioning_defaults() -> Self {
        Self {
            aec: Some(true),
            audio_codec: Some(AudioCodec::Pcm),
            audio_response_type: Some(AudioResponseType::Tts),
            bss: Some(false),
            command_endpoint: Some(CommandEndpoint::HomeAssistant),
            display_timeout: Some(10),
            hass_host: Some("homeassistant.local".into()),
            hass_port: Some(8123),
            hass_tls: Some(false),
            hass_token: Some("your_ha_token".into()),
            lcd_brightness: Some(500),
            lvgl_lock_timeout: None,
            mic_gain: Some(14),
            mqtt_auth_type: Some(MqttAuthType::UserPassword),
            mqtt_host: Some("your.mqtt.host".into()),
            mqtt_password: Some("your_mqtt_password".into()),
            mqtt_port: Some(1883),
            mqtt_tls: Some(false),
            mqtt_topic: Some("your_mqtt_topic".into()),
            mqtt_username: Some("your_mqtt_username".into()),
            multiwake: Some(false),
            ntp_config: Some(NtpConfig::Host),
            ntp_host: Some("pool.ntp.org".into()),
            openhab_token: Some("your_openhab_token".into()),
            openhab_url: Some("your_openhab_url".into()),
            record_buffer: Some(12),
            rest_auth_header: Some("your_header".into()),
            rest_auth_pass: Some("your_password".into()),
            rest_auth_type: Some(RestAuthType::None),
            rest_auth_user: Some("your_username".into()),
            rest_url: Some("http://your_rest_url".into()),
            show_prereleases: Some(false),
            speaker_volume: Some(60),
            speech_rec_mode: Some(SpeechRecognitionMode::Wis),
            stream_timeout: Some(5),
            timezone: Some("UTC+5".into()),
            timezone_name: Some("America/Menominee".into()),
            vad_mode: Some(VadMode::Mode2),
            vad_timeout: Some(300),
            wake_confirmation: Some(false),
            wake_mode: Some(WakeMode::TwoChannel95),
            wake_word: Some("alexa".into()),
            was_mode: None,
            wis_tts_url: Some("https://infer.tovera.io/api/tts".into()),
            wis_tts_url_v2: None,
            wis_url: Some("https://infer.tovera.io/api/willow".into()),
        }
    }
}
