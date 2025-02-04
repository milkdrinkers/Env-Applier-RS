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

use crate::env::parser;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_env_file_paths(base_path: &Path, mode: &str) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut add_if_exists = |filename: &str| {
        let path = base_path.join(filename);
        if path.exists() {
            files.push(path);
        }
    };

    // Base files
    add_if_exists(".env");
    add_if_exists(".env.local");

    // Mode-specific files
    if !mode.is_empty() {
        add_if_exists(&format!(".env.{}", mode));
        add_if_exists(&format!(".env.{}.local", mode));
    }

    Ok(files)
}

pub fn load_and_parse_files(files: &[PathBuf]) -> Result<HashMap<String, String>> {
    let mut all_vars = HashMap::new();

    for path in files {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        parser::parse_contents(&content, &mut all_vars)
            .with_context(|| format!("Failed to parse file: {}", path.display()))?;
    }

    Ok(all_vars)
}
