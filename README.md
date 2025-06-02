CLI functionality is simple. It reads a config that specifies a list of files and nodes. The user can then apply or unapply patches.

## Command Line Usage

`env-applier-rs` or (*ea*) is a CLI tool for applying environment variables in configuration files.

### Basic Usage

```bash
ea [COMMAND] [OPTIONS]
```

### Commands

#### `apply`
Applies the configuration by replacing variables in the target files with values from the environment.

```bash
ea apply [OPTIONS]

Options:
  -c, --config <FILE>  Path to config file
  -h, --help          Print help information
```

#### `deapply`
Reverts the configuration by restoring the original values in the target files.

```bash
ea deapply [OPTIONS]

Options:
  -c, --config <FILE>  Path to config file
  -h, --help          Print help information
```

#### `parse`
Validates the configuration file without making any changes to the target files.

```bash
ea parse [OPTIONS]

Options:
  -c, --config <FILE>  Path to config file
  -h, --help          Print help information
```

### Global Options

```bash
-h, --help       Print help information
-V, --version    Print version information
```

### Examples

Apply configuration using the default config file (`config.toml`):
```bash
ea apply
```

Apply configuration using a specific config file:
```bash
ea apply --config ./config.toml
```

Validate a specific config file:
```bash
ea parse --config ./some-config.toml
```

Revert changes using the default config file (`config.toml`):
```bash
ea deapply
```

### Notes

- If no config file is specified, the tool will look for the default configuration file `config.toml` in your current working directory
- The tool will load environment variables from your system, `.env` & `.env.local`
- The parse command can be used to validate config files before applying changes

### Configuration

The configuration file is what tells Env Applier what to change and where. It should be a single `Toml` file located in the *current working directory* or specified using the command argument for config.

By default `config.toml` is loaded by the CLI.

```toml
[environment]
# In this case the de-apply output would be something like "%%SOME_ENV_NAME%%"
prefix = "%%" # What env variables should be prefixed with when changes are de-applied
suffix = "%%" # What env variables should be suffixed with when changes are de-applied

[specific]
    [specific.json]
    [specific.toml]
    [specific.properties]
    [specific.hocon]
    [specific.yaml]
        [[specific.yaml.locations]]
        file = "test.yml" # The file which contains the config node
        node = "database.password" # The config node to change
        variable = "DB_PASS" # This is the env variable that is parsed on apply, and replaced back on de-apply

        [[specific.yaml.locations]]
        file = [ "test.yml", "test2.yml", ] # Allows multiple files to be updated
        node = "database.host" 
        variable = "DB_HOST"

        [[specific.yaml.locations]]
        file = "test.yml"
        node = [ "database.name", ] # Allow multiple nodes to be updated
        variable = "DB_NAME"

        [[specific.yaml.locations]]
        files = "test.yml" # Alias of "file"
        nodes = "server.password" # Alias of "node"
        variable = "DB_PASS"
        default = "%%OVERRIDE_DEAPPLY%%" # Overrides the "variable" field when de-applying
        override = { exemptApply = false, exemptDeapply = false } # Exempt from applying or de-applying changes

        [[specific.yaml.locations]]
        files = [ "test.yml", ] # Alias of "file"
        nodes = [ "database.password", "server.password" ] # Alias of "node"
        variable = "DB_PASS"
        default = "%%DB_CUSTOM_VARIABLE%%"
        override = { exemptApply = false, exemptDeapply = false }
```

### Environment Variables

Environment variables are loaded from your system and the files `.env` & `.env.local` (*Located in your current working directory*).

`.env` files follow `.properties` format and support variable expnasion. Here is an example `.env` file which makes the following environment variables available to the program:
```properties
# SQL Database Settings
DB_NAME=database
DB_USER=root
DB_PASS=123
DB_HOST=localhost
DB_PORT=3306

# Example of variable expansion
DB_URL=mysql://$DB_USER:$DB_PASS@$DB_ADDRESS/$DB_NAME
DB_ADDRESS=$DB_HOST:$DB_PORT
```

### File Formats

| Format         | Comments Support   | Table Support | Array Support |
|----------------|--------------------|---------------|---------------|
| **YAML**       | ✅ (Using `#`)      | ✅ (Maps)     | ✅ (Sequences) |
| **JSON**       | ❌                  | ❌            | ✅             |
| **JSONC**      | ✅ (Using `//`)     | ❌            | ✅             |
| **TOML**       | ✅ (Using `#`)      | ✅            | ✅             |
| **HOCON**      | ✅ (Using `#`/`//`) | ✅         | ✅             |
| **Properties** | ✅ (Using `#`/`!`)  | ❌         | ❌             |

The specs & criteria for modifications is straightforward:
- Only replaces the value of a node, it never adds entries to the file
- Does not "*reformat*" the file at all
- Respects previous whitespace/indentation/line endings
- Respects comments in applicable config formats (*Including header, footer & trailing comments*)
- Able to traverse & modify complex data structures in supported file formats
- Editing a file should never break the syntax of the config
