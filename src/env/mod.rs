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

pub mod loader;
pub mod parser;

use std::collections::HashMap;
use std::env;
use std::path::Path;

pub fn load() -> Result<(), anyhow::Error> {
    let mode = get_current_mode();
    let files = loader::get_env_file_paths(Path::new("."), &mode)?;
    let variables = loader::load_and_parse_files(&files)?;

    set_environment_variables(variables);
    Ok(())
}

fn get_current_mode() -> String {
    env::var("BUN_ENV")
        .or_else(|_| env::var("NODE_ENV"))
        .unwrap_or_else(|_| "development".into())
}

fn set_environment_variables(vars: HashMap<String, String>) {
    for (key, value) in vars {
        if env::var(&key).is_err() {
            env::set_var(key, value);
        }
    }
}
