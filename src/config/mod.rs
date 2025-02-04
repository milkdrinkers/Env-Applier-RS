/*
 * MIT License
 *
 * Copyright (c) 2025 darksaid98
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;
use tokio::fs;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Config file not found at {0}")]
    NotFound(String),
    #[error("Config parsing error: {0}")]
    ParseError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Config {
    #[serde(default)]
    pub environment: Environment,
    #[serde(default)]
    pub specific: Specific,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Environment {
    #[serde(default = "default_prefix")]
    pub prefix: String,
    #[serde(default = "default_suffix")]
    pub suffix: String,
    #[serde(default)]
    pub variables: Vec<String>,
}

fn default_prefix() -> String {
    "%".to_string()
}
fn default_suffix() -> String {
    "%".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Specific {
    #[serde(default)]
    pub json: FileTypeConfig,
    #[serde(default)]
    pub toml: FileTypeConfig,
    #[serde(default)]
    pub yaml: FileTypeConfig,
    #[serde(default)]
    pub properties: FileTypeConfig,
    #[serde(default)]
    pub xml: FileTypeConfig,
}

impl Specific {
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &FileTypeConfig)> {
        vec![
            ("json", &self.json),
            ("toml", &self.toml),
            ("yaml", &self.yaml),
            ("properties", &self.properties),
            ("xml", &self.xml),
        ]
        .into_iter()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FileTypeConfig {
    #[serde(default)]
    pub locations: Vec<Location>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    #[serde(
        deserialize_with = "deserialize_paths",
        default,
        alias = "files",  // Add alias for plural form
        rename = "file"   // But keep original name in struct
    )]
    pub file: Vec<PathBuf>,
    #[serde(
        deserialize_with = "deserialize_nodes",
        default,
        alias = "nodes",  // Add alias for plural form
        rename = "node"   // But keep original name in struct
    )]
    pub node: Vec<String>,

    pub variable: String,
    pub default: Option<String>,

    #[serde(default, rename = "override")]
    pub override_settings: OverrideSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OverrideSettings {
    #[serde(rename = "exemptApply")]
    pub exempt_apply: bool,
    #[serde(rename = "exemptDeapply")]
    pub exempt_deapply: bool,
}

fn deserialize_paths<'de, D>(deserializer: D) -> Result<Vec<PathBuf>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum PathsInput {
        Single(String),
        Multiple(Vec<String>),
    }

    let input = Option::<PathsInput>::deserialize(deserializer)?;

    Ok(match input {
        Some(PathsInput::Single(s)) => vec![PathBuf::from(s)],
        Some(PathsInput::Multiple(v)) => v.into_iter().map(PathBuf::from).collect(),
        None => Vec::new(),
    })
}

fn deserialize_nodes<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum NodesInput {
        Single(String),
        Multiple(Vec<String>),
    }

    let input = Option::<NodesInput>::deserialize(deserializer)?;

    Ok(match input {
        Some(NodesInput::Single(s)) => vec![s],
        Some(NodesInput::Multiple(v)) => v,
        None => Vec::new(),
    })
}

pub async fn load_config(custom_path: Option<PathBuf>) -> Result<Config, ConfigError> {
    let config_path = custom_path.unwrap_or_else(|| PathBuf::from("config.toml"));

    if !config_path.exists() {
        return Err(ConfigError::NotFound(
            config_path.to_string_lossy().to_string(),
        ));
    }

    let config_str = fs::read_to_string(&config_path).await?;
    let config: Config = toml::from_str(&config_str)
        .map_err(|e| ConfigError::ParseError(format!("TOML parsing error: {}", e)))?;

    Ok(config)
}
