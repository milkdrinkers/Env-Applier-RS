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

/// Updates a value in a JSON file while preserving structure and formatting.
///
/// # Arguments
/// * `file_path` - Path to the JSON file
/// * `node_path` - Dot-notation path to the target node (e.g., "parent.child")
/// * `new_value` - The new value to set
///
/// # Notes
/// * Preserves comments (// style)
/// * Maintains original indentation
/// * Keeps trailing commas
/// * Only modifies values, not structure
pub async fn update_json_node(file_path: &Path, node_path: &str, new_value: &str) -> Result<()> {
    // Read file content and split into lines
    let json_content = fs::read_to_string(file_path).await?;
    let mut lines: Vec<String> = json_content.lines().map(|d| d.to_string()).collect();
    let path_parts: Vec<&str> = node_path.split('.').collect();

    // Detect the file's indentation style from the first indented line
    let mut indent_size = 2;
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.is_empty() {
            continue;
        }
        if let Some(space_count) = line
            .chars()
            .take_while(|c| c.is_whitespace())
            .count()
            .checked_sub(0)
        {
            if space_count > 0 {
                indent_size = space_count;
                break;
            }
        }
    }

    let mut current_depth = 0;
    let mut path_index = 0;
    let mut target_line = None;

    // Find the target line by traversing the path
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.is_empty() {
            continue;
        }

        // Match key-value pairs considering possible trailing comma and comment
        let expected_indent = " ".repeat(current_depth);
        let re = Regex::new(&format!(
            r#"^{}\s*"{}"\s*:\s*(.*?)(,?)\s*(//.*)?"#,
            regex::escape(&expected_indent),
            regex::escape(path_parts[path_index])
        ))?;

        if let Some(_captures) = re.captures(line) {
            if path_index == path_parts.len() - 1 {
                target_line = Some(i);
                break;
            }
            path_index += 1;
            current_depth += indent_size;
        }
    }

    // Update the target line if found
    if let Some(target_index) = target_line {
        let line = &lines[target_index];

        // Preserve any trailing comment
        let comment_re = Regex::new(r"(//.*)$")?;
        let comment = comment_re
            .captures(line)
            .map_or("".to_string(), |caps| caps[1].to_string());

        // Preserve trailing comma if present
        let comma_re = Regex::new(r",\s*(//.*)?")?;
        let has_comma = comma_re.is_match(line);

        let indent = " ".repeat(current_depth);
        lines[target_index] = format!(
            "{}\"{}\": {}{}{}",
            indent,
            path_parts[path_index],
            new_value,
            if has_comma { "," } else { "" },
            comment
        );
    }

    // Write the modified content back to the file
    fs::write(file_path, lines.join("\n")).await?;
    Ok(())
}
