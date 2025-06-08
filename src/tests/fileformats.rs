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

mod test_utils {
    use anyhow::Result;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    pub fn create_test_file(temp_dir: &TempDir, filename: &str, content: &str) -> Result<PathBuf> {
        let file_path = temp_dir.path().join(filename);
        fs::write(&file_path, content)?;
        Ok(file_path)
    }
}

// YAML Tests
#[cfg(test)]
mod yaml_tests {
    use crate::tests::fileformats::test_utils::create_test_file;
    use crate::utils::yaml::update_yaml_node;
    use anyhow::Result;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_basic_yaml_update() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "test.yaml",
            r#"# Header comment
name: old_value # Inline comment
other: value
# Footer comment"#,
        )?;

        update_yaml_node(&file_path, "name", "new_value").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("name: new_value"));
        assert!(content.contains("# Header comment"));
        assert!(content.contains("# Inline comment"));
        assert!(content.contains("# Footer comment"));
        Ok(())
    }

    #[tokio::test]
    async fn test_nested_yaml_update() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "nested.yaml",
            r#"database:
  host: localhost
  port: 5432
  credentials:
    username: old_user # Important user
    password: secret
  options:
    timeout: 30"#,
        )?;

        update_yaml_node(&file_path, "database.credentials.username", "new_user").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("username: new_user"));
        assert!(content.contains("# Important user"));
        assert!(content.contains("password: secret"));
        Ok(())
    }

    #[tokio::test]
    async fn test_yaml_array_preservation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "array.yaml",
            r#"items:
  - name: item1
    value: 42
  - name: item2
    value: 84
target: old_value # Change this
sequences:
  - [1, 2, 3]
  - [4, 5, 6]"#,
        )?;

        update_yaml_node(&file_path, "target", "new_value").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("target: new_value"));
        assert!(content.contains("name: item1"));
        assert!(content.contains("name: item2"));
        assert!(content.contains("- [1, 2, 3]"));
        Ok(())
    }

    #[tokio::test]
    async fn test_yaml_complex_indentation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "indentation.yaml",
            r#"mapping:
    key1: value1    # Four space indent
    nested:
        deep:
            target: old_value    # Eight space indent
            other: value
        sibling: value"#,
        )?;

        update_yaml_node(&file_path, "mapping.nested.deep.target", "new_value").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("            target: new_value"));
        assert!(content.contains("sibling: value"));
        Ok(())
    }

    #[tokio::test]
    async fn test_yaml_multiline_strings() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "multiline.yaml",
            r#"literals:
  folded: >
    This is a
    folded text
    block
  literal: |
    This is a
    literal text
    block
  target: old_value # Update this
  flow: {key: value, other: value} # Flow style"#,
        )?;

        update_yaml_node(&file_path, "literals.target", "new_value").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("target: new_value"));
        assert!(content.contains("folded: >"));
        assert!(content.contains("literal: |"));
        assert!(content.contains("flow: {key: value, other: value}"));
        Ok(())
    }

    #[tokio::test]
    async fn test_yaml_special_values() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "special.yaml",
            r#"special:
  null_value: null
  bool_value: true
  target: "old_value" # Quoted string
  reference: &ref_value
    key: value
  alias: *ref_value
  date: 2024-02-01"#,
        )?;

        update_yaml_node(&file_path, "special.target", "\"new_value\"").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("target: \"new_value\""));
        assert!(content.contains("null_value: null"));
        assert!(content.contains("&ref_value"));
        assert!(content.contains("*ref_value"));
        Ok(())
    }

    #[tokio::test]
    async fn test_yaml_empty_values() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "empty.yaml",
            r#"empty_values:
  empty_string: ""
  target: old_value
  empty_map: {}
  empty_list: []
  explicit_null: null"#,
        )?;

        update_yaml_node(&file_path, "empty_values.target", "\"\"").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("target: \"\""));
        assert!(content.contains("empty_map: {}"));
        assert!(content.contains("empty_list: []"));
        Ok(())
    }

    #[tokio::test]
    async fn test_yaml_comments_preservation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "comments.yaml",
            r#"# Configuration file
# Last updated: 2024-02-01

settings: # Main settings section
  # Database settings
  database:
    host: localhost # Default host
    target: old_value # Target value to change
    # End of database section

  # Application settings
  app:
    port: 8080

# End of configuration"#,
        )?;

        update_yaml_node(&file_path, "settings.database.target", "new_value").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("target: new_value # Target value to change"));
        assert!(content.contains("# Configuration file"));
        assert!(content.contains("# Database settings"));
        assert!(content.contains("# End of configuration"));
        Ok(())
    }
}

// JSON Tests
#[cfg(test)]
mod json_tests {
    use crate::tests::fileformats::test_utils::create_test_file;
    use crate::utils::json::update_json_node;
    use anyhow::Result;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_basic_json_update() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "test.json",
            r#"{
    // Header comment
    "name": "old_value", // Inline comment
    "nested": {
        "key": "value"
    }
    // Footer comment
}"#,
        )?;

        update_json_node(&file_path, "name", "\"new_value\"").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("\"name\": \"new_value\""));
        assert!(content.contains("// Header comment"));
        assert!(content.contains("// Inline comment"));
        assert!(content.contains("// Footer comment"));
        Ok(())
    }

    #[tokio::test]
    async fn test_nested_json_update() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "nested.json",
            r#"{
    "level1": {
        "level2": {
            "target": "old_value", // Keep this comment
            "sibling": "unchanged"
        }
    }
}"#,
        )?;

        update_json_node(&file_path, "level1.level2.target", "\"new_value\"").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("\"target\": \"new_value\""));
        assert!(content.contains("// Keep this comment"));
        assert!(content.contains("\"sibling\": \"unchanged\""));
        Ok(())
    }

    #[tokio::test]
    async fn test_json_array_preservation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "array.json",
            r#"{
    "arrays": {
        "simple": [1, 2, 3],
        "target": "old_value",
        "complex": [
            {"key": "value"},
            {"key": "value2"}
        ]
    }
}"#,
        )?;

        update_json_node(&file_path, "arrays.target", "\"new_value\"").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("\"target\": \"new_value\""));
        assert!(content.contains("\"simple\": [1, 2, 3]"));
        assert!(content.contains("\"complex\": ["));
        Ok(())
    }
}

// TOML Tests
#[cfg(test)]
mod toml_tests {
    use crate::tests::fileformats::test_utils::create_test_file;
    use crate::utils::toml::update_toml_node;
    use anyhow::Result;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_basic_toml_update() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "test.toml",
            r#"# Header comment
title = "old_value" # Inline comment
[section]
key = "value"
# Footer comment"#,
        )?;

        update_toml_node(&file_path, "title", "\"new_value\"").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("title = \"new_value\""));
        assert!(content.contains("# Header comment"));
        assert!(content.contains("# Inline comment"));
        assert!(content.contains("# Footer comment"));
        Ok(())
    }

    #[tokio::test]
    async fn test_nested_toml_update() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "nested.toml",
            r#"[database]
host = "localhost"
port = 5432

[database.credentials]
username = "old_user" # Important user
password = "secret"

[other]
key = "value""#,
        )?;

        update_toml_node(&file_path, "database.credentials.username", "\"new_user\"").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("username = \"new_user\""));
        assert!(content.contains("# Important user"));
        assert!(content.contains("password = \"secret\""));
        Ok(())
    }

    #[tokio::test]
    async fn test_toml_array_preservation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "array.toml",
            r#"[[test.items]]
name = "item1"
value = 42

[some]
target = "old_value2"  #Change 2this

[[test.items]]
name = "item2"
value = 84

[test]
target = "old_value" # Change this"#,
        )?;

        update_toml_node(&file_path, "some.target", "\"old_value2\"").await?;
        update_toml_node(&file_path, "test.target", "\"new_value\"").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains(r#"target = "old_value2"  #Change 2this"#));
        assert!(content.contains(r#"target = "new_value" # Change this"#));
        assert!(content.contains("[[test.items]]"));
        assert!(content.contains(r#"name = "item1""#));
        assert!(content.contains(r#"name = "item2""#));
        Ok(())
    }
}

// Properties Tests
#[cfg(test)]
mod properties_tests {
    use crate::tests::fileformats::test_utils::create_test_file;
    use crate::utils::properties::update_properties_node;
    use anyhow::Result;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_basic_properties_update() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "test.properties",
            r#"# Header comment
key = old_value # Inline comment
other.key = value
# Footer comment"#,
        )?;

        update_properties_node(&file_path, "key", "new_value").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("key = new_value"));
        assert!(content.contains("# Header comment"));
        assert!(content.contains("# Inline comment"));
        assert!(content.contains("# Footer comment"));
        Ok(())
    }

    #[tokio::test]
    async fn test_properties_special_chars() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "special.properties",
            r#"# Special characters
path.to.key = old:value # Has colon
url.key = http://example.com
space.key = old value with spaces"#,
        )?;

        update_properties_node(&file_path, "path.to.key", "new:value").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("path.to.key = new:value"));
        assert!(content.contains("# Has colon"));
        assert!(content.contains("url.key = http://example.com"));
        Ok(())
    }

    #[tokio::test]
    async fn test_properties_multiline() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "multiline.properties",
            r#"# Multi-line value
long.key = old \
    value \
    continues # Comment
regular.key = value"#,
        )?;

        update_properties_node(
            &file_path,
            "long.key",
            "new \\\n    value \\\n    continues",
        )
            .await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("long.key = new \\"));
        assert!(content.contains("    continues # Comment"));
        assert!(content.contains("regular.key = value"));
        Ok(())
    }

    #[tokio::test]
    async fn test_whitespace_preservation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "whitespace.properties",
            r#"# Test whitespace preservation
no_spaces=old_value
single_space = old_value
multiple_spaces     =     old_value
tabs	=	old_value
mixed   =	  old_value"#,
        )?;

        // Test each case
        update_properties_node(&file_path, "no_spaces", "new_value").await?;
        update_properties_node(&file_path, "single_space", "new_value").await?;
        update_properties_node(&file_path, "multiple_spaces", "new_value").await?;
        update_properties_node(&file_path, "tabs", "new_value").await?;
        update_properties_node(&file_path, "mixed", "new_value").await?;

        let content = std::fs::read_to_string(&file_path)?;

        // Verify exact whitespace preservation
        assert!(content.contains("no_spaces=new_value"));
        assert!(content.contains("single_space = new_value"));
        assert!(content.contains("multiple_spaces     =     new_value"));
        assert!(content.contains("tabs\t=\tnew_value"));
        assert!(content.contains("mixed   =\t  new_value"));
        Ok(())
    }

    #[tokio::test]
    async fn test_no_quotes_added() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "quotes.properties",
            r#"# Test that quotes are not added
host = old_host
port=old_port
url = http://example.com"#,
        )?;

        update_properties_node(&file_path, "host", "135.148.171.219:27072").await?;
        update_properties_node(&file_path, "port", "8080").await?;
        update_properties_node(&file_path, "url", "https://newsite.com").await?;

        let content = std::fs::read_to_string(&file_path)?;

        // Verify no quotes are added
        assert!(content.contains("host = 135.148.171.219:27072"));
        assert!(content.contains("port=8080"));
        assert!(content.contains("url = https://newsite.com"));

        // Verify no quotes exist around values
        assert!(!content.contains("\"135.148.171.219:27072\""));
        assert!(!content.contains("\"8080\""));
        assert!(!content.contains("\"https://newsite.com\""));
        Ok(())
    }

    #[tokio::test]
    async fn test_empty_values() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "empty.properties",
            r#"# Test empty values
voice_host = old_value
empty_key =
another_key=something"#,
        )?;

        update_properties_node(&file_path, "voice_host", "").await?;
        update_properties_node(&file_path, "empty_key", "").await?;
        update_properties_node(&file_path, "another_key", "").await?;

        let content = std::fs::read_to_string(&file_path)?;

        // Verify empty values are handled correctly (no quotes added)
        // Each should preserve its original spacing around =
        assert!(content.contains("voice_host = "));  // Had space after =
        assert!(content.contains("empty_key ="));   // Had space after =
        assert!(content.contains("another_key="));   // No space after =

        // Verify no empty quotes
        assert!(!content.contains("\"\""));
        Ok(())
    }

    #[tokio::test]
    async fn test_values_with_spaces() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "spaces.properties",
            r#"# Test values with spaces
name = old name
path = /old/path with spaces
description=old description here"#,
        )?;

        update_properties_node(&file_path, "name", "new name with spaces").await?;
        update_properties_node(&file_path, "path", "/new/path with more spaces").await?;
        update_properties_node(&file_path, "description", "new description here").await?;

        let content = std::fs::read_to_string(&file_path)?;

        // Verify values with spaces work without quotes
        assert!(content.contains("name = new name with spaces"));
        assert!(content.contains("path = /new/path with more spaces"));
        assert!(content.contains("description=new description here"));
        Ok(())
    }

    #[tokio::test]
    async fn test_values_with_special_characters() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "special.properties",
            r#"# Test special characters in values
url = http://old.com
regex = old.*pattern
json = {"old": "value"}"#,
        )?;

        update_properties_node(&file_path, "url", "http://new.com:8080/path?param=value").await?;
        update_properties_node(&file_path, "regex", "new.*pattern[a-z]+").await?;
        update_properties_node(&file_path, "json", r#"{"new": "value", "array": [1,2,3]}"#).await?;

        let content = std::fs::read_to_string(&file_path)?;

        // Verify special characters work without quotes
        assert!(content.contains("url = http://new.com:8080/path?param=value"));
        assert!(content.contains("regex = new.*pattern[a-z]+"));
        assert!(content.contains(r#"json = {"new": "value", "array": [1,2,3]}"#));
        Ok(())
    }

    #[tokio::test]
    async fn test_comments_with_equals() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "comment_equals.properties",
            r#"# Test comments with equals signs
key = old_value # This comment has = signs in it
another=value # URL = http://example.com"#,
        )?;

        update_properties_node(&file_path, "key", "new_value").await?;
        update_properties_node(&file_path, "another", "new_another").await?;

        let content = std::fs::read_to_string(&file_path)?;

        // Verify comments with equals are preserved
        assert!(content.contains("key = new_value # This comment has = signs in it"));
        assert!(content.contains("another=new_another # URL = http://example.com"));
        Ok(())
    }

    #[tokio::test]
    async fn test_key_not_found() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "missing.properties",
            r#"# Test missing key
existing_key = value
# Comment line"#,
        )?;

        // This should not modify the file since key doesn't exist
        update_properties_node(&file_path, "nonexistent_key", "new_value").await?;

        let content = std::fs::read_to_string(&file_path)?;

        // Verify file is unchanged
        assert!(content.contains("existing_key = value"));
        assert!(!content.contains("nonexistent_key"));
        assert!(!content.contains("new_value"));
        Ok(())
    }
}

// HOCON Tests
#[cfg(test)]
mod hocon_tests {
    use crate::tests::fileformats::test_utils::create_test_file;
    use crate::utils::hocon::update_hocon_node;
    use anyhow::Result;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_hocon_update_simple_key_value() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "simple.conf",
            r#"
# This is a comment
database = "localhost"
port = 5432
enabled = true
"#,
        )?;

        update_hocon_node(&file_path, "database", "remote-host").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("database = \"remote-host\""));
        assert!(content.contains("port = 5432"));
        assert!(content.contains("# This is a comment"));
        Ok(())
    }

    #[tokio::test]
    async fn test_update_hocon_nested_object() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "nested.conf",
            r#"
server {
  host = "localhost"
  port = 8080
  ssl {
    enabled = false
    cert = "path/to/cert"
  }
}
"#,
        )?;

        update_hocon_node(&file_path, "server.ssl.enabled", "true").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("enabled = true"));
        assert!(content.contains("host = \"localhost\""));
        Ok(())
    }

    #[tokio::test]
    async fn test_hocon_update_with_comments() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "comments.conf",
            r#"
# Database configuration
database {
  host = "localhost" # This is the host
  port = 5432 // This is the port
}
"#,
        )?;

        update_hocon_node(&file_path, "database.host", "remote").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("host = \"remote\" # This is the host"));
        assert!(content.contains("// This is the port"));
        Ok(())
    }

    #[tokio::test]
    async fn test_hocon_complex_update() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "complex.conf",
            r###"
# Application Configuration
app {
    name = "MyApp"
    version: "1.0.0"  // version comment

    server {
        "bind-address" = "127.0.0.1"
        port: 8080

        security {
            oauth {
                "client-id" = "test-client" # client identifier
                secret = "secret-value"

                endpoints {
                    // Token endpoint configuration
                    "token-url" = "https://old.token.url" # needs update
                    auth-url: "https://auth.url"
                }
            }
        }
    }

    features {
        experimental = false
        "user-management" = true
    }
}
"###,
        )?;

        // Test criteria from your image:
        // 1. Deep nesting: app.server.security.oauth.endpoints.token-url
        // 2. Mixed key formats (quoted/unquoted)
        // 3. Mixed assignment operators (= vs :)
        // 4. Multiple comment styles
        update_hocon_node(
            &file_path,
            "app.server.security.oauth.endpoints.token-url",
            "\"https://new.token.url\"",
        )
            .await?;

        let content = std::fs::read_to_string(&file_path)?;

        // Verify all preservation criteria
        // 1. Target value updated with proper quoting
        assert!(content.contains(r#""token-url" = "https://new.token.url""#));

        // 2. Deep nesting structure preserved
        assert!(content.contains("endpoints {"));
        assert!(content.contains("oauth {"));

        // 3. Mixed key formats preserved
        assert!(content.contains(r#""bind-address""#));      // Quoted key
        assert!(content.contains("port:"));                   // Unquoted key
        assert!(content.contains(r#""client-id""#));         // Quoted key
        assert!(content.contains(r#""token-url""#));          // Quoted key

        // 4. Mixed assignment operators preserved
        assert!(content.contains(r#"name = "MyApp""#));       // Equals operator
        assert!(content.contains("version: \"1.0.0\""));     // Colon operator
        assert!(content.contains(r#"secret = "secret-value""#));
        assert!(content.contains("auth-url: \"https://auth.url\""));

        // 5. Comment preservation
        assert!(content.contains("# Application Configuration"));  // Full-line hash
        assert!(content.contains("// version comment"));           // Full-line double-slash
        assert!(content.contains("# client identifier"));          // End-of-line hash
        assert!(content.contains("// Token endpoint configuration")); // Full-line double-slash
        assert!(content.contains("# needs update"));               // End-of-line hash

        // 6. Unchanged values preservation
        assert!(content.contains(r#""bind-address" = "127.0.0.1""#));
        assert!(content.contains("port: 8080"));
        assert!(content.contains(r#""client-id" = "test-client""#));
        assert!(content.contains("experimental = false"));
        assert!(content.contains(r#""user-management" = true"#));

        // 7. Structure preservation
        assert!(content.contains("features {"));
        assert!(content.contains("security {"));

        Ok(())
    }

    // Additional edge case tests
    #[tokio::test]
    async fn test_hocon_deep_nesting_with_mixed_formats() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "deep_nesting.conf",
            r#"
level1 {
    "level2-key" = {
        level3: {
            "level3-key" = "old" # comment
            "level4-key": {
                "target.key" = "old.value"  # c2omment
            }
        }
    }
}
"#,
        )?;

        update_hocon_node(
            &file_path,
            "level1.level2-key.level3.level3-key",
            "\"new.value\"",
        )
            .await?;
        update_hocon_node(
            &file_path,
            "level1.\"level2-key\".level3.\"level4-key\".\"target.key\"",
            "\"new.value\"",
        )
            .await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains(r#""level3-key" = "new.value""#));
        assert!(content.contains("# comment"));
        assert!(content.contains(r#""target.key" = "new.value""#));
        assert!(content.contains("  # c2omment"));
        assert!(content.contains("\"level2-key\" = {"));
        assert!(content.contains("level3: {"));

        Ok(())
    }

    #[tokio::test]
    async fn test_hocon_preserve_whitespace_and_comments() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "whitespace.conf",
            r#"
key1 = value1   # end comment

// Section comment
section {
    key2: value2   // with comment
    key3 = value3
}
"#,
        )?;

        update_hocon_node(&file_path, "section.key2", "\"new_value\"").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("key2: \"new_value\"   // with comment"));
        assert!(content.contains("key1 = value1   # end comment"));
        assert!(content.contains("// Section comment"));
        assert!(content.contains("key3 = value3"));

        // Verify whitespace preservation
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines[0], "");
        assert_eq!(lines[1], "key1 = value1   # end comment");
        assert_eq!(lines[2], "");
        assert_eq!(lines[3], "// Section comment");
        assert_eq!(lines[4], "section {");
        assert_eq!(lines[5], "    key2: \"new_value\"   // with comment");
        assert_eq!(lines[6], "    key3 = value3");
        assert_eq!(lines[7], "}");

        Ok(())
    }

    #[tokio::test]
    async fn test_hocon_keys_with_special_chars() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "special_chars.conf",
            r#"
"key:with:colon" = "value1"
"key.with.dots" = "value2"
"key with spaces" = "value3"
"#,
        )?;

        // Test colon in key
        update_hocon_node(&file_path, "\"key:with:colon\"", "\"new1\"").await?;

        // Test dots in key
        update_hocon_node(&file_path, "\"key.with.dots\"", "\"new2\"").await?;

        // Test spaces in key
        update_hocon_node(&file_path, "\"key with spaces\"", "\"new3\"").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains(r#""key:with:colon" = "new1""#));
        assert!(content.contains(r#""key.with.dots" = "new2""#));
        assert!(content.contains(r#""key with spaces" = "new3""#));
        Ok(())
    }

    #[tokio::test]
    async fn test_hocon_value_with_comment_chars() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(
            &temp_dir,
            "comment_chars.conf",
            r#"
key1 = value1   # end comment

// Section comment
section {
    key2: "jdbc:mysql://localhost:3306/test"   // with comment
    key3 = "jdbc:mysql://localhost:3306/test"
}
"#,
        )?;

        update_hocon_node(&file_path, "key1", "jdbc:mysql://127.0.0.1:3306/name").await?;
        update_hocon_node(&file_path, "section.key2", "jdbc:mysql://127.0.0.1:3306/name").await?;
        update_hocon_node(&file_path, "section.key3", "jdbc:mysql://127.0.0.1:3306/name").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("key1 = \"jdbc:mysql://127.0.0.1:3306/name\"   # end comment"));
        assert!(content.contains("key2: \"jdbc:mysql://127.0.0.1:3306/name\"   // with comment"));
        assert!(content.contains("// Section comment"));
        assert!(content.contains("key3 = \"jdbc:mysql://127.0.0.1:3306/name\""));

        Ok(())
    }
}

// XML Tests
/*#[cfg(test)]
mod xml_tests {
    use anyhow::Result;
    use crate::utils::xml::update_xml_node;
    use tempfile::TempDir;
    use crate::tests::fileformats::test_utils::create_test_file;

    #[tokio::test]
    async fn test_basic_xml_update() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(&temp_dir, "test.xml", r#"<?xml version="1.0" encoding="UTF-8"?>
<!-- Header comment -->
<root>
    <target>old_value</target> <!-- Inline comment -->
    <other>value</other>
</root>
<!-- Footer comment -->"#)?;

        update_xml_node(&file_path, "root.target", "new_value").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("<target>new_value</target>"));
        assert!(content.contains("<!-- Header comment -->"));
        assert!(content.contains("<!-- Inline comment -->"));
        assert!(content.contains("<!-- Footer comment -->"));
        Ok(())
    }

    #[tokio::test]
    async fn test_nested_xml_update() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(&temp_dir, "nested.xml", r#"<?xml version="1.0"?>
<config>
    <database>
        <connection>
            <host>localhost</host>
            <port>5432</port>
            <target>old_value</target> <!-- Important -->
        </connection>
    </database>
</config>"#)?;

        update_xml_node(&file_path, "config.database.connection.target", "new_value").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("<target>new_value</target>"));
        assert!(content.contains("<!-- Important -->"));
        assert!(content.contains("<host>localhost</host>"));
        Ok(())
    }

    #[tokio::test]
    async fn test_xml_attributes_preservation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = create_test_file(&temp_dir, "attributes.xml", r#"<?xml version="1.0"?>
<root xmlns:custom="https://example.com">
    <elem id="1" class="test">
        <target>old_value</target>
        <other custom:attr="value">text</other>
    </elem>
</root>"#)?;

        update_xml_node(&file_path, "root.elem.target", "new_value").await?;

        let content = std::fs::read_to_string(&file_path)?;
        assert!(content.contains("<target>new_value</target>"));
        assert!(content.contains("xmlns:custom=\"https://example.com\""));
        assert!(content.contains("custom:attr=\"value\""));
        Ok(())
    }
}*/
