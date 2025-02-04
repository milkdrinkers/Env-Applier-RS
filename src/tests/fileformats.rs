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
