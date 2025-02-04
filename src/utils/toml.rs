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

use anyhow::Result;
use regex::Regex;
use std::path::Path;
use tokio::fs;

/// Updates a value in a TOML file while preserving structure and formatting.
///
/// # Arguments
/// * `file_path` - Path to the TOML file
/// * `node_path` - Dot-notation path to the target node (e.g., "parent.child")
/// * `new_value` - The new value to set
///
/// # Notes
/// * Preserves comments (# style)
/// * Maintains original indentation
/// * Keeps trailing commas
/// * Only modifies values, not structure
pub async fn update_toml_node(file_path: &Path, node_path: &str, new_value: &str) -> Result<()> {
    let toml_content = fs::read_to_string(file_path).await?;
    let mut lines: Vec<String> = toml_content.lines().map(|s| s.to_string()).collect();
    let path_parts: Vec<&str> = node_path.split('.').collect();

    let mut current_array = Vec::new();
    let mut current_table = Vec::new();
    let mut target_line = None;

    // Traverse the file looking for the target node
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }

        // Track the current table or array context
        if trimmed.starts_with('[') {
            if trimmed.starts_with("[[") && trimmed.ends_with("]]") {
                // Array of tables (e.g., `[[items]]`)
                let array_re = Regex::new(r"^\[\[([^\]]+)\]\]$")?;
                if let Some(captures) = array_re.captures(trimmed) {
                    current_array = captures[1].split('.').map(|s| s.to_string()).collect();
                }
            } else {
                // Regular table (e.g., `[table]`)
                let table_re = Regex::new(r"^\[([^\]]+)\]$")?;
                if let Some(captures) = table_re.captures(trimmed) {
                    current_table = captures[1].split('.').map(|s| s.to_string()).collect();
                    current_array.clear(); // Reset array context when entering a new table
                }
            }
            continue;
        }

        // Match key-value pairs preserving original spacing
        let key_re = Regex::new(&format!(
            r#"^(\s*{}\s*=\s*)(["'].*?["']|[^#\s]+)(\s*#.*)?$"#,
            regex::escape(path_parts.last().unwrap_or(&""))
        ))?;

        if let Some(captures) = key_re.captures(line) {
            // Construct full path including table and array context
            let full_path = if !current_array.is_empty() {
                // If inside an array, include the array context
                let mut full_path_parts = current_table.clone();
                full_path_parts.extend(current_array.clone());
                full_path_parts.push(path_parts.last().unwrap().to_string());
                full_path_parts.join(".")
            } else if !current_table.is_empty() {
                // If inside a table, include the table context
                let mut full_path_parts = current_table.clone();
                full_path_parts.push(path_parts.last().unwrap().to_string());
                full_path_parts.join(".")
            } else {
                // Top-level key
                path_parts.last().unwrap().to_string()
            };

            if full_path == node_path {
                target_line = Some((
                    i,
                    captures[1].to_string(),
                    captures.get(3).map(|m| m.as_str().to_string()),
                ));
                break;
            }
        }
    }

    // Update the target line if found, preserving original formatting
    if let Some((target_index, prefix, comment)) = target_line {
        lines[target_index] = format!(
            "{}{}{}",
            prefix,
            new_value,
            comment.map(|c| format!("{}", c)).unwrap_or_default()
        );
    }

    fs::write(file_path, lines.join("\n")).await?;
    Ok(())
}
