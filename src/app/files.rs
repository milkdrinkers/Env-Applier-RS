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
use crate::config::Config;
use anyhow::Result;
use std::collections::HashSet;

pub async fn get(config: &Config) -> Result<HashSet<String>> {
    let mut unique_paths = HashSet::new();

    for (_file_format, file_config) in config.specific.iter() {
        for loc in &file_config.locations {
            if loc.override_settings.exempt_apply {
                continue;
            }

            if loc.file.is_empty() {
                continue;
            }

            if let Ok(_environment_variable) = std::env::var(&loc.variable) {} else {
                continue;
            };

            for file in &loc.file {
                if !file.exists() {
                    continue;
                }

                if loc.node.is_empty() {
                    continue;
                }

                for node in &loc.node {
                    if node.is_empty() || node.trim().is_empty() {
                        continue;
                    }

                    unique_paths.insert(file.to_string_lossy().to_string());
                }
            }
        }
    }

    Ok(unique_paths)
}
