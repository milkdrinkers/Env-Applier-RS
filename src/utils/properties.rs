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
use std::path::Path;
use tokio::fs;

/// Updates a value in a .properties file while preserving structure and formatting.
///
/// # Arguments
/// * `file_path` - Path to the properties file
/// * `key` - The key to update
/// * `new_value` - The new value to set
///
/// # Notes
/// * Handles flat key-value structure
/// * Preserves comments (# style)
/// * Maintains original formatting
/// * Preserves spacing around = separator
pub async fn update_properties_node(file_path: &Path, key: &str, new_value: &str) -> Result<()> {
    let content = fs::read_to_string(file_path).await?;
    let mut lines: Vec<String> = content.lines().map(|d| d.to_string()).collect();
    let mut target_line = None;

    // Find the line containing the target key
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }

        // Simple approach: find key at start, then find = and capture parts
        if let Some(eq_pos) = line.find('=') {
            let key_part = line[..eq_pos].trim();
            if key_part == key {
                target_line = Some(i);
                break;
            }
        }
    }

    // Update the target line if found
    if let Some(target_index) = target_line {
        let line = &lines[target_index];

        // Find the = position
        if let Some(eq_pos) = line.find('=') {
            // Extract the key portion with its whitespace
            let key_with_whitespace = &line[..eq_pos];

            // Find where the value starts after =
            let after_eq = &line[eq_pos + 1..];

            // Find if there's a comment (# that's not part of the value)
            let mut comment_start = None;
            let mut in_quotes = false;
            let mut escape_next = false;

            for (i, ch) in after_eq.char_indices() {
                if escape_next {
                    escape_next = false;
                    continue;
                }

                match ch {
                    '\\' => escape_next = true,
                    '"' => in_quotes = !in_quotes,
                    '#' if !in_quotes => {
                        comment_start = Some(i);
                        break;
                    }
                    _ => {}
                }
            }

            // Extract the part after = but before any comment
            let (value_part_with_spaces, comment_part) = if let Some(comment_pos) = comment_start {
                let value_part = &after_eq[..comment_pos];
                let comment_part = &after_eq[comment_pos..];
                (value_part, comment_part)
            } else {
                (after_eq, "")
            };

            // Find the whitespace pattern around the value
            let leading_spaces = &value_part_with_spaces[..value_part_with_spaces.len() - value_part_with_spaces.trim_start().len()];

            // For trailing spaces, we need to be careful when the new value is empty
            // If new_value is empty, we want to preserve the leading spaces but not add trailing spaces
            let trailing_spaces = if new_value.is_empty() {
                ""
            } else {
                &value_part_with_spaces[value_part_with_spaces.trim_end().len()..]
            };

            // Reconstruct the line
            lines[target_index] = format!("{}={}{}{}{}",
                                          key_with_whitespace,
                                          leading_spaces,
                                          new_value,
                                          trailing_spaces,
                                          comment_part
            );
        }
    }

    fs::write(file_path, lines.join("\n")).await?;
    Ok(())
}
