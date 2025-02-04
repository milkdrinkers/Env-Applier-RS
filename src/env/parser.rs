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

use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::HashMap;

pub fn parse_contents(contents: &str, vars: &mut HashMap<String, String>) -> Result<()> {
    let line_re = Regex::new(r"(?m)^\s*([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(.*?)(\s*#.*)?$")?;
    // Handle both ${VAR} and $VAR syntax
    let expand_re = Regex::new(r"\$(?:\{([A-Za-z_][A-Za-z0-9_]*)\}|([A-Za-z_][A-Za-z0-9_]*))")?;

    // First pass: collect raw variables
    let mut raw_vars = Vec::new();
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let caps = line_re
            .captures(line)
            .ok_or_else(|| anyhow!("Invalid line format: {}", line))?;

        let key = caps[1].trim().to_string();
        let raw_value = caps[2].trim();

        // Skip if already set in environment
        if std::env::var(&key).is_ok() {
            continue;
        }

        raw_vars.push((key, raw_value.to_string()));
    }

    // Insert all variables with unquoted values first
    for (key, raw_value) in &raw_vars {
        vars.insert(key.clone(), unquote_value(raw_value));
    }

    // Multi-pass expansion until no more changes
    let mut iterations = 0;
    let max_iterations = 15; // Prevent circular referencing forever
    let mut changed = true;

    while changed && iterations < max_iterations {
        changed = false;
        iterations += 1;

        // Expand variables with support for both syntaxes
        for (key, value) in vars.clone() {
            let expanded = expand_re
                .replace_all(&value, |caps: &regex::Captures| {
                    let var_name = caps
                        .get(1)
                        .or_else(|| caps.get(2))
                        .map(|m| m.as_str())
                        .unwrap_or("");

                    vars.get(var_name)
                        .map(|v| v.clone())
                        .or_else(|| std::env::var(var_name).ok().map(|s| s))
                        .unwrap_or(String::new())
                        .to_owned()
                })
                .to_string();

            if expanded != value {
                vars.insert(key.clone(), expanded);
                changed = true;
            }
        }
    }

    if iterations >= max_iterations {
        println!("Possible circular reference detected in environment variables!");
    }

    Ok(())
}

fn unquote_value(value: &str) -> String {
    let value = value.trim();
    if value.len() < 2 {
        return value.to_string();
    }

    let quote = value.chars().next().unwrap();
    if quote != '"' && quote != '\'' {
        return value.to_string();
    }

    if value.chars().last().unwrap() != quote {
        return value.to_string();
    }

    value[1..value.len() - 1].to_string()
}
