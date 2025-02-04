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

mod apply;
mod deapply;

pub use apply::apply;
pub use deapply::deapply;
use std::path::PathBuf;

async fn change_file(
    file_type: &'static str,
    file: &PathBuf,
    node: &str,
    value: &str,
) -> anyhow::Result<()> {
    if file_type == "yaml" {
        crate::utils::yaml::update_yaml_node(file, node, value).await?;
    } else if file_type == "json" {
        crate::utils::json::update_json_node(file, node, value).await?;
    } else if file_type == "toml" {
        crate::utils::toml::update_toml_node(file, node, value).await?;
    } else if file_type == "xml" {
        crate::utils::xml::update_xml_node(file, node, value).await?;
    } else if file_type == "properties" {
        crate::utils::properties::update_properties_node(file, node, value).await?;
    }

    Ok(())
}
