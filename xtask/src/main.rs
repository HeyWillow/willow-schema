//! Repository maintenance tasks.

use std::{env, fs, io, path::Path};

use schemars::{JsonSchema, Schema, generate::SchemaSettings};
use serde_json::Value;
use willow_schema::{config, nvs};

const OUTPUT_DIRECTORY: &str = "generated/json-schema";

struct Document {
    file_name: &'static str,
    schema: Schema,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    match arguments
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["json-schema"] => generate_json_schema(false),
        ["json-schema", "--check"] => generate_json_schema(true),
        _ => Err("usage: cargo xtask json-schema [--check]".into()),
    }
}

fn generate_json_schema(check: bool) -> Result<(), Box<dyn std::error::Error>> {
    let documents = [
        Document {
            file_name: "config-v1.schema.json",
            schema: generate::<config::v1::Config>("urn:heywillow:schema:config:v1"),
        },
        Document {
            file_name: "nvs-v1.schema.json",
            schema: generate::<nvs::v1::Config>("urn:heywillow:schema:nvs:v1"),
        },
    ];

    let output_directory = workspace_root().join(OUTPUT_DIRECTORY);
    if !check {
        fs::create_dir_all(&output_directory)?;
    }

    let mut stale = Vec::new();
    for document in documents {
        let path = output_directory.join(document.file_name);
        let mut expected = serde_json::to_string_pretty(&document.schema)?;
        expected.push('\n');

        if check {
            match fs::read_to_string(&path) {
                Ok(actual) if actual == expected => {}
                Ok(_) | Err(_) => stale.push(path),
            }
        } else {
            write_if_changed(&path, expected.as_bytes())?;
        }
    }

    if stale.is_empty() {
        Ok(())
    } else {
        let paths = stale
            .iter()
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        Err(format!("generated JSON Schema is stale or missing: {paths}").into())
    }
}

fn generate<T: JsonSchema>(id: &str) -> Schema {
    let generator = SchemaSettings::draft2020_12()
        .for_deserialize()
        .into_generator();
    let mut schema = generator.into_root_schema_for::<T>();
    schema
        .ensure_object()
        .insert("$id".into(), Value::String(id.into()));
    schema
}

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must be a direct child of the workspace root")
}

fn write_if_changed(path: &Path, expected: &[u8]) -> io::Result<()> {
    if fs::read(path).is_ok_and(|actual| actual == expected) {
        return Ok(());
    }

    fs::write(path, expected)
}
