# mule-lazy-migrate

A robust, user-friendly CLI tool to automate migration of Mule 4 projects to a new runtime using a JSON config.

## Features
- Reads a JSON config with upgrade rules (runtime, plugin, munit versions, artifact JSON, string replacements)
- Traverses the project, updates XML (`pom.xml`), JSON (`mule-artifact.json`), and performs string replacements in source files
- Supports dry-run and file backups
- Modular and testable codebase
- Colorized, human-friendly summary of changes at the end
- Optional Maven integration: update dependencies and build after migration

## Usage

```sh
cargo run --release -- \
  --config runtime_configs/migration-4.9.4.json \
  --project-root /path/to/your/mule-project \
  --backup
```

### Options
- `--config <path>`: Path to the migration JSON config
- `--project <path>`: Path to the Mule project root
- `--dry-run`: Preview changes without modifying files
- `--backup`: Create `.bak` backups before modifying files
- `-u`, `--update-maven-deps`: Run `mvn versions:use-latest-releases` before migration
- `-b`, `--build-mule-project`: Run `mvn clean install` after migration

### Example (all options)
```sh
cargo run --release -- \
  --config runtime_configs/migration-4.9.4.json \
  --project-root /path/to/your/mule-project \
  --backup -u -b
```

## Output
At the end of each run, a colorized summary is printed, showing all changes, warnings, and errors.

## Requirements
- Rust (latest stable)
- Java & Maven (for Maven integration)

## License
MIT 