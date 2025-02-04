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

        let re = Regex::new(&format!(r"^{}\s*=\s*(.*?)(\s*#.*)?$", regex::escape(key)))?;

        if re.is_match(line) {
            target_line = Some(i);
            break;
        }
    }

    // Update the target line if found
    if let Some(target_index) = target_line {
        let line = &lines[target_index];
        let comment_re = Regex::new(r"(\s*#.*)$")?;
        let comment = comment_re
            .captures(line)
            .map_or("".to_string(), |caps| caps[1].to_string());

        lines[target_index] = format!("{} = {}{}", key, new_value, comment);
    }

    fs::write(file_path, lines.join("\n")).await?;
    Ok(())
}
