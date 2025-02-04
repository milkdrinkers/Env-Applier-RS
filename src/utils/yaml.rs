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
use std::str::FromStr;
use std::string::String;
use tokio::fs;

pub async fn update_yaml_node(file_path: &Path, node_path: &str, new_value: &str) -> Result<()> {
    let yaml_content = fs::read_to_string(file_path).await?;
    let mut lines: Vec<String> = yaml_content.lines().map(|d| d.to_string()).collect();
    let path_parts: Vec<&str> = node_path.split('.').collect();

    // Detect indentation from first non-empty line
    let mut indent_size = 2;
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.is_empty() {
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

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }

        let expected_indent = " ".repeat(current_depth);
        let re = regex::Regex::new(&format!(
            r"^{}(\S+):\s*(.*)",
            regex::escape(&expected_indent)
        ))
        .unwrap();

        if let Some(captures) = re.captures(line) {
            if captures.get(1).map_or("", |m| m.as_str()) == path_parts[path_index] {
                if path_index == path_parts.len() - 1 {
                    target_line = Some(i);
                    break;
                }
                path_index += 1;
                current_depth += indent_size;
            }
        }
    }

    if let Some(target_index) = target_line {
        let line = &lines[target_index];
        let comment_re = regex::Regex::new(r"(\s*#.*)$").unwrap();
        let comment = comment_re
            .captures(line)
            .map_or("".to_string(), |caps| caps[1].to_string());
        let indent = " ".repeat(current_depth);

        lines[target_index] = format!(
            "{}{}: {}{}",
            indent, path_parts[path_index], new_value, comment
        );
    }

    fs::write(file_path, lines.join("\n")).await?;
    Ok(())
}

pub fn parse_variable(env: &str) -> String {
    if let Ok(numeric_value) = f64::from_str(env) {
        if !env.trim().is_empty() {
            return numeric_value.to_string();
        }
    }
    format!("\"{}\"", env)
}
