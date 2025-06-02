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

pub async fn update_hocon_node(file_path: &Path, node_path: &str, new_value: &str) -> Result<()> {
    let hocon_content = fs::read_to_string(file_path).await?;

    // Capture trailing newlines
    let trailing_newlines: String = hocon_content
        .chars()
        .rev()
        .take_while(|c| *c == '\n')
        .collect::<String>()
        .chars()
        .rev()
        .collect();

    let main_content = if trailing_newlines.is_empty() {
        hocon_content.as_str()
    } else {
        &hocon_content[0..hocon_content.len() - trailing_newlines.len()]
    };

    let mut lines: Vec<String> = main_content.lines().map(|d| d.to_string()).collect();
    let path_parts = parse_hocon_path(node_path);

    // Detect indentation from first non-empty line
    let mut indent_size = 2;
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.starts_with("//") || trimmed.is_empty() {
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
    let mut depth_stack = Vec::new(); // Track nesting depth

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Skip comments and empty lines
        if trimmed.starts_with('#') || trimmed.starts_with("//") || trimmed.is_empty() {
            continue;
        }

        // Calculate current depth based on indentation
        let line_indent = line.len() - line.trim_start().len();

        // Adjust current depth based on indentation changes
        while !depth_stack.is_empty() && line_indent <= *depth_stack.last().unwrap() {
            depth_stack.pop();
            if path_index > 0 {
                path_index -= 1;
            }
        }
        current_depth = line_indent;

        // Handle different HOCON key-value formats
        let key_value_patterns = [
            // key = value (including key = {)
            r"^(\s*)([a-zA-Z_][a-zA-Z0-9_-]*)\s*=\s*(.*)$",
            // key: value (including key: {)
            r"^(\s*)([a-zA-Z_][a-zA-Z0-9_-]*)\s*:\s*(.*)$",
            // "key" = value (including "key" = {)
            r#"^(\s*)"([^"]+)"\s*=\s*(.*)$"#,
            // "key": value (including "key": {)
            r#"^(\s*)"([^"]+)"\s*:\s*(.*)$"#,
            // key { (object start on same line)
            r"^(\s*)([a-zA-Z_][a-zA-Z0-9_-]*)\s*\{\s*(.*)$",
            // "key" { (quoted object start on same line)
            r#"^(\s*)"([^"]+)"\s*\{\s*(.*)$"#,
        ];

        let mut matched_key = None;
        let mut matched_indent = 0;
        let mut matched_value = String::new();
        let mut is_object = false;

        for pattern in &key_value_patterns {
            let re = Regex::new(pattern).unwrap();
            if let Some(captures) = re.captures(line) {
                matched_indent = captures.get(1).map_or("", |m| m.as_str()).len();
                matched_key = Some(captures.get(2).map_or("", |m| m.as_str()));
                matched_value = captures.get(3).map_or("", |m| m.as_str()).to_string();

                // Check if this is an object (either explicit { or value contains {)
                is_object = pattern.contains(r"\{") || matched_value.trim_start().starts_with('{');
                break;
            }
        }

        if let Some(key) = matched_key {
            // Check if this matches our current path part at the correct depth
            if matched_indent == current_depth &&
                path_index < path_parts.len() &&
                key == path_parts[path_index] {
                if path_index == path_parts.len() - 1 {
                    // Found our target key-value pair
                    target_line = Some(i);
                    break;
                } else {
                    // This is part of our path, continue deeper
                    path_index += 1;
                    depth_stack.push(matched_indent);

                    // If this is an object, we expect the next level to be indented
                    if is_object {
                        current_depth = matched_indent + indent_size;
                    }
                }
            }
        }
    }

    if let Some(target_index) = target_line {
        let line = &lines[target_index];

        // Preserve comments at the end of the line
        let comment_patterns = [
            r"(\s*#.*)$",     // # comments
            r"(\s*//.*)$",    // // comments
        ];

        let mut comment = String::new();
        for pattern in &comment_patterns {
            let comment_re = Regex::new(pattern).unwrap();
            if let Some(caps) = comment_re.captures(line) {
                comment = caps[1].to_string();
                break;
            }
        }

        let indent = " ".repeat(current_depth);
        let key = &path_parts[path_index];

        // Format the new value based on its type
        let formatted_value = format_hocon_value(new_value);

        // Determine the assignment operator to use based on the original line
        let assignment_op = if line.contains(" = ") || line.contains("=") {
            " = "
        } else {
            ": "
        };

        // Handle quoted keys if the original was quoted
        let formatted_key = if line.trim_start().starts_with('"') {
            format!("\"{}\"", key)
        } else {
            key.to_string()
        };

        lines[target_index] = format!(
            "{}{}{}{}{}",
            indent, formatted_key, assignment_op, formatted_value, comment
        );
    }

    let new_main_content = lines.join("\n");
    let new_content = format!("{}{}", new_main_content, trailing_newlines);

    fs::write(file_path, new_content).await?;
    Ok(())
}

fn parse_hocon_path(path: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current_part = String::new();
    let mut in_quotes = false;
    let mut chars = path.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '"' if !in_quotes => {
                in_quotes = true;
                // Don't include the quote in the key name
            }
            '"' if in_quotes => {
                in_quotes = false;
                // Don't include the quote in the key name
            }
            '.' if !in_quotes => {
                if !current_part.is_empty() {
                    parts.push(current_part.clone());
                    current_part.clear();
                }
            }
            '\\' if in_quotes => {
                // Handle escaped characters in quoted strings
                if let Some(next_ch) = chars.next() {
                    current_part.push(next_ch);
                }
            }
            _ => {
                current_part.push(ch);
            }
        }
    }

    if !current_part.is_empty() {
        parts.push(current_part);
    }

    parts
}

fn format_hocon_value(value: &str) -> String {
    // If the value is already quoted or is a number/boolean, use as-is
    if value.starts_with('"') && value.ends_with('"') {
        return value.to_string();
    }

    // Check if it's a number
    if value.parse::<f64>().is_ok() {
        return value.to_string();
    }

    // Check if it's a boolean
    if value.eq_ignore_ascii_case("true") || value.eq_ignore_ascii_case("false") {
        return value.to_lowercase();
    }

    // Check if it's null
    if value.eq_ignore_ascii_case("null") {
        return "null".to_string();
    }

    // For everything else, quote it as a string
    format!("\"{}\"", value)
}
