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

use anyhow::anyhow;
use anyhow::Result;
use regex::Regex;
use std::path::Path;
use std::str::FromStr;
use std::string::String;
use tokio::fs;

pub async fn update_xml_node(file_path: &Path, node_path: &str, new_value: &str) -> Result<()> {
    let xml_content = fs::read_to_string(file_path).await?;
    let mut lines: Vec<String> = xml_content.lines().map(|d| d.to_string()).collect();
    let path_parts: Vec<&str> = node_path.split('.').collect();

    // Track current state
    let mut _current_depth = 0;
    let mut path_index = 0;
    let mut in_target_path = false;
    let mut target_line = None;
    let mut found_nodes = Vec::new();

    // Regex patterns
    let opening_tag_re = Regex::new(r"^(\s*)<([^>/\s][^>\s]*)[^>]*>([^<]*)").unwrap();
    let closing_tag_re = Regex::new(r"^(\s*)</([^>\s]+)\s*>").unwrap();
    let self_closing_re = Regex::new(r"^(\s*)<([^>/\s][^>\s]*)[^>]*/\s*>").unwrap();
    let comment_re = Regex::new(r"(\s*<!--.*?-->)").unwrap();

    // First pass: detect indentation
    let mut indent_size = 2;
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("<!--") || trimmed.is_empty() {
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

    // Second pass: find and update target node
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Skip comments and empty lines
        if trimmed.is_empty() {
            continue;
        }

        // Handle comments
        if comment_re.is_match(trimmed) {
            continue;
        }

        // Handle self-closing tags
        if let Some(captures) = self_closing_re.captures(line) {
            let tag_name = captures.get(2).unwrap().as_str();
            if in_target_path && tag_name == path_parts[path_index] {
                if path_index == path_parts.len() - 1 {
                    // Can't update self-closing tags
                    return Err(anyhow!("Cannot update self-closing tags"));
                }
            }
            continue;
        }

        // Handle closing tags
        if let Some(captures) = closing_tag_re.captures(line) {
            let tag_name = captures.get(2).unwrap().as_str();
            if in_target_path && tag_name == *found_nodes.last().unwrap_or(&"") {
                found_nodes.pop();
                if found_nodes.is_empty() {
                    in_target_path = false;
                }
                _current_depth -= indent_size;
            }
            continue;
        }

        // Handle opening tags
        if let Some(captures) = opening_tag_re.captures(line) {
            let _indent = captures.get(1).map_or("", |m| m.as_str());
            let tag_name = captures.get(2).unwrap().as_str();
            let _content = captures.get(3).map_or("", |m| m.as_str()).trim();

            if !in_target_path && tag_name == path_parts[path_index] {
                found_nodes.push(tag_name);
                path_index += 1;
                in_target_path = true;
                _current_depth += indent_size;

                if path_index == path_parts.len() {
                    target_line = Some(i);
                    break;
                }
            } else if in_target_path && tag_name == path_parts[path_index] {
                found_nodes.push(tag_name);
                path_index += 1;
                _current_depth += indent_size;

                if path_index == path_parts.len() {
                    target_line = Some(i);
                    break;
                }
            }
        }
    }

    // Update the target line while preserving formatting
    if let Some(target_index) = target_line {
        let line = &lines[target_index];
        let opening_tag_re = Regex::new(r"^(\s*<[^>]+>)([^<]*)(</[^>]+>)(.*)$").unwrap();

        if let Some(captures) = opening_tag_re.captures(line) {
            let prefix = captures.get(1).unwrap().as_str();
            let suffix = captures.get(3).unwrap().as_str();
            let trailing = captures.get(4).map_or("", |m| m.as_str());

            lines[target_index] = format!("{}{}{}{}", prefix, new_value, suffix, trailing);
        }
    }

    fs::write(file_path, lines.join("\n")).await?;
    Ok(())
}

pub fn _parse_variable(env: &str) -> String {
    if let Ok(numeric_value) = f64::from_str(env) {
        if !env.trim().is_empty() {
            return numeric_value.to_string();
        }
    }
    env.to_string()
}
