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

#[cfg(test)]
mod tests {
    use crate::config::Config;
    use std::path::PathBuf;

    const SAMPLE_CONFIG: &str = r##"
        [environment]
        prefix = "test"
        suffix = "test"
        variables = ["APP_ENV", "DEBUG"]

        [specific.yaml]
            [[specific.yaml.locations]]
            file = "single.yml"
            node = "single.node"
            variable = "VAR1"

            [[specific.yaml.locations]]
            file = ["multi1.yml", "multi2.yml"]
            nodes = ["node1", "node2"]
            variable = "VAR2"

            [[specific.yaml.locations]]
            files = ["both_files1.yml", "both_files2.yml"]
            nodes = ["both_nodes1", "both_nodes2"]
            variable = "VAR3"
            override = { exemptApply = true, exemptDeapply = false }
    "##;

    #[tokio::test]
    async fn test_full_config_parsing() {
        let config: Config = toml::from_str(SAMPLE_CONFIG).unwrap();

        // Test environment section
        assert_eq!(config.environment.prefix, "test");
        assert_eq!(config.environment.suffix, "test");
        assert_eq!(config.environment.variables, vec!["APP_ENV", "DEBUG"]);

        // Test yaml locations
        let yaml_locations = &config.specific.yaml.locations;
        assert_eq!(yaml_locations.len(), 3);

        // First location
        assert_eq!(yaml_locations[0].file, vec![PathBuf::from("single.yml")]);
        assert_eq!(yaml_locations[0].node, vec!["single.node"]);
        assert_eq!(yaml_locations[0].variable, "VAR1");
        assert!(yaml_locations[0].default.is_none());

        // Second location
        assert_eq!(
            yaml_locations[1].file,
            vec![PathBuf::from("multi1.yml"), PathBuf::from("multi2.yml")]
        );
        assert_eq!(yaml_locations[1].node, vec!["node1", "node2"]);
        assert_eq!(yaml_locations[1].variable, "VAR2");

        // Third location (merged fields)
        assert_eq!(
            yaml_locations[2].file,
            vec![
                PathBuf::from("both_files1.yml"),
                PathBuf::from("both_files2.yml")
            ]
        );
        assert_eq!(yaml_locations[2].node, vec!["both_nodes1", "both_nodes2"]);
        assert_eq!(yaml_locations[2].variable, "VAR3");
        assert!(yaml_locations[2].override_settings.exempt_apply);
        assert!(!yaml_locations[2].override_settings.exempt_deapply);
    }

    #[test]
    fn test_default_values() {
        let toml_input = r#"
            [specific.properties]
                [[specific.properties.locations]]
                variable = "DEFAULT_TEST"
        "#;

        let config: Config = toml::from_str(toml_input).unwrap();
        let location = &config.specific.properties.locations[0];

        assert!(location.file.is_empty());
        assert!(location.node.is_empty());
        assert_eq!(location.variable, "DEFAULT_TEST");
        assert!(!location.override_settings.exempt_apply);
        assert!(!location.override_settings.exempt_deapply);
    }

    #[test]
    fn test_invalid_toml() {
        let invalid_toml = r#"
            [specific.yaml.locations]
            file = "test.yml"
            node = "server.port
            variable = "PORT"
        "#; // Missing closing quote for node

        let result: Result<Config, _> = toml::from_str(invalid_toml);
        assert!(result.is_err());
    }

    #[test]
    fn test_edge_cases() {
        let toml_input = r#"
            [specific.yaml]
                [[specific.yaml.locations]]
                files = []
                nodes = []
                variable = "EMPTY_TEST"

                [[specific.yaml.locations]]
                file = ""
                node = ""
                variable = "BLANK_TEST"
        "#;

        let config: Config = toml::from_str(toml_input).unwrap();

        let empty_location = &config.specific.yaml.locations[0];
        assert!(empty_location.file.is_empty());
        assert!(empty_location.node.is_empty());

        let blank_location = &config.specific.yaml.locations[1];
        assert_eq!(blank_location.file, vec![PathBuf::from("")]);
        assert_eq!(blank_location.node, vec![""]);
    }
}
