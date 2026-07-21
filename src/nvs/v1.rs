//! The existing Willow NVS provisioning wire format.

use alloc::string::String;
use core::fmt;
use serde::{Deserialize, Serialize};

/// The complete Willow NVS provisioning v1 JSON document.
///
/// Each known section and property is required because Willow cannot connect
/// to Wi-Fi or its application server without these values. Unknown object
/// properties are accepted during deserialization but are not retained, so
/// this typed representation does not promise lossless round-tripping.
///
/// This type models the serialized provisioning document shared by Willow and
/// WAS. It does not model ESP-IDF NVS storage operations. `v1` is the Rust
/// module version; no version property is added to the wire document.
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "WillowNvsConfigV1"))]
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    /// Configures the Willow Application Server connection.
    #[serde(rename = "WAS")]
    pub was: Was,
    /// Configures the Wi-Fi connection.
    #[serde(rename = "WIFI")]
    pub wifi: Wifi,
}

/// Error returned when a Wi-Fi WPA passphrase violates the v1 contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WifiPskError {
    /// The passphrase is shorter than 8 bytes or longer than 63 bytes.
    InvalidLength,
    /// The passphrase contains a byte outside printable ASCII.
    NonPrintableAscii,
}

impl fmt::Display for WifiPskError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength => formatter.write_str("WPA passphrase must be 8 to 63 bytes"),
            Self::NonPrintableAscii => {
                formatter.write_str("WPA passphrase must contain only printable ASCII")
            }
        }
    }
}

/// A validated Wi-Fi WPA passphrase.
///
/// The v1 firmware accepts WPA passphrases containing 8 through 63 printable
/// ASCII bytes. The owned string is allocator-backed.
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "json-schema",
    schemars(extend(
        "minLength" = 8,
        "maxLength" = 63,
        "pattern" = r"^[\x20-\x7E]+$",
        "x-willow-length-unit" = "bytes"
    ))
)]
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
#[serde(try_from = "String")]
pub struct WifiPsk(String);

impl WifiPsk {
    /// Borrows the validated passphrase.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the owned validated passphrase.
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl TryFrom<String> for WifiPsk {
    type Error = WifiPskError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !(8..=63).contains(&value.len()) {
            return Err(WifiPskError::InvalidLength);
        }
        if !value
            .bytes()
            .all(|byte| byte.is_ascii_graphic() || byte == b' ')
        {
            return Err(WifiPskError::NonPrintableAscii);
        }
        Ok(Self(value))
    }
}

/// Error returned when a Wi-Fi SSID violates the v1 contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WifiSsidError {
    /// The SSID is empty or longer than 32 bytes.
    InvalidLength,
}

impl fmt::Display for WifiSsidError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Wi-Fi SSID must be 1 to 32 bytes")
    }
}

/// A validated Wi-Fi service set identifier.
///
/// The v1 firmware stores SSIDs containing 1 through 32 bytes. The owned
/// string is allocator-backed, so it can represent UTF-8 SSIDs within that
/// byte limit.
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "json-schema",
    schemars(extend(
        "minLength" = 1,
        "maxLength" = 32,
        "x-willow-max-utf8-bytes" = 32
    ))
)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(try_from = "String")]
pub struct WifiSsid(String);

impl WifiSsid {
    /// Borrows the validated SSID.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the owned validated SSID.
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl TryFrom<String> for WifiSsid {
    type Error = WifiSsidError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > 32 {
            return Err(WifiSsidError::InvalidLength);
        }
        Ok(Self(value))
    }
}

/// Willow Application Server NVS namespace values.
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct Was {
    /// Sets the WebSocket URL used to connect to WAS.
    #[serde(rename = "URL")]
    pub url: String,
}

/// Wi-Fi NVS namespace values.
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct Wifi {
    /// Sets the WPA passphrase.
    #[serde(rename = "PSK")]
    pub psk: WifiPsk,
    /// Sets the service set identifier.
    #[serde(rename = "SSID")]
    pub ssid: WifiSsid,
}
