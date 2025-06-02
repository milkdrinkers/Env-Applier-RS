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
use std::fs;
use crate::app::change_file;
use crate::config::Config;
use crate::utils::yaml::parse_variable;
use anyhow::Result;
use filetime::{set_file_times, FileTime};

pub async fn deapply(config: &Config) -> Result<u32> {
    let mut changes = 0;

    for (file_format, file_config) in config.specific.iter() {
        for loc in &file_config.locations {
            if loc.override_settings.exempt_deapply {
                continue;
            }

            if loc.file.is_empty() {
                continue;
            }

            let replacement = if let Some(node) = &loc.default {
                node
            } else {
                &format!(
                    "{}{}{}",
                    &config.environment.prefix, &loc.variable, &config.environment.suffix
                )
            };

            for file in &loc.file {
                if !file.exists() {
                    continue;
                }

                if loc.node.is_empty() {
                    continue;
                }

                let mut file_changed = false;
                let original_metadata = fs::metadata(file)?;
                let original_mtime = FileTime::from_last_modification_time(&original_metadata);
                let original_atime = FileTime::from_last_access_time(&original_metadata);

                for node in &loc.node {
                    if node.is_empty() || node.trim().is_empty() {
                        continue;
                    }

                    change_file(
                        file_format,
                        file,
                        node,
                        parse_variable(&replacement).as_str(),
                    )
                    .await?;
                    file_changed = true;
                    changes += 1;
                }

                // Preserve original file metadata
                if file_changed {
                    set_file_times(file, original_atime, original_mtime)?;
                }
            }
        }
    }

    Ok(changes)
}
