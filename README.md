# mule-lazy-migrate

A robust, user-friendly CLI tool to automate migration of Mule 4 projects to a new runtime using a JSON config.

**Current Version: v0.1.3**

## Features
- Reads a JSON config with upgrade rules (runtime, plugin, munit versions, artifact JSON, string replacements)
- Traverses the project, updates XML (`pom.xml`), JSON (`mule-artifact.json`), and performs string replacements in source files
- Supports dry-run and file backups
- Modular and testable codebase
- Colorized, human-friendly summary of changes at the end
- Optional Maven integration: update dependencies and build after migration
- **Supports only flat `javaSpecificationVersions` at the root of `mule-artifact.json`**
- **Verbose logging** for detailed debugging and troubleshooting

## ⚠️ Important: Dependency Version Updates
- This tool **does NOT update** `<version>` tags inside `<dependency>` blocks in `pom.xml`.
- If you use the `--update-maven-deps` (`-u`) flag, Maven's `versions:use-latest-releases` will update dependency versions based on what is available in your Maven repositories.
- The version chosen by Maven may not be the latest if your repositories are missing newer versions.
- To avoid automatic dependency version changes, **do not use** the `--update-maven-deps` flag.

## Installation

### Homebrew (macOS/Linux - recommended)

```sh
brew tap kchernokozinsky/mule-lazy-migrate
brew install mule-lazy-migrate
```

### From Source

#### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Git](https://git-scm.com/)

#### Installation Steps

```sh
# Clone the repository
git clone https://github.com/kchernokozinsky/mule-lazy-migrate.git
cd mule-lazy-migrate

# Build and install
cargo install --path .
```

### Windows Installation

For Windows users, install from source:

```powershell
# Install Rust from https://rustup.rs/
# Install Git from https://git-scm.com/

# Clone and install
git clone https://github.com/kchernokozinsky/mule-lazy-migrate.git
cd mule-lazy-migrate
cargo install --path .
```

## Usage

```sh
mule-lazy-migrate \
  --config runtime_configs/migration-4.9.4.json \
  --project /path/to/your/mule-project \
  --backup
```

### Options
- `--config <path>`: Path to the migration JSON config
- `--project <path>`: Path to the Mule project root
- `--dry-run`: Preview changes without modifying files
- `--backup`: Create `.bak` backups before modifying files
- `-u`, `--update-maven-deps`: Run `mvn versions:use-latest-releases` before migration (see warning above)
- `-b`, `--build-mule-project`: Run `mvn clean install` after migration
- `-v`, `--verbose`: Show debug logs for detailed troubleshooting
- `-V`, `--version`: Display version information and exit

### Example (all options)
```sh
mule-lazy-migrate \
  --config runtime_configs/migration-4.9.4.json \
  --project /path/to/your/mule-project \
  --backup -u -b -v
```

### Verbose Logging
Use the `--verbose` flag to enable detailed debug logging. This is useful for:
- Troubleshooting migration issues
- Understanding exactly what files are being processed
- Seeing detailed information about XML and JSON updates
- Debugging string replacement operations
- Monitoring Maven integration steps

Example with verbose output:
```sh
mule-lazy-migrate --config config.json --verbose
```

### Version Information
Check the installed version:
```sh
mule-lazy-migrate --version
```

## Output
At the end of each run, a colorized summary is printed, showing all changes, warnings, and errors.

## Requirements
- **Rust** (latest stable) - for building from source
- **Java & Maven** - for Maven integration features
- **Git** - for cloning the repository

## License
MIT

# homebrew-mule-lazy-migrate

This is a [Homebrew](https://brew.sh/) tap for the `mule-lazy-migrate` CLI tool.

## How to use

First, add this tap:

```sh
brew tap kchernokozinsky/mule-lazy-migrate
```

Then install the tool:

```sh
brew install mule-lazy-migrate
```

## About mule-lazy-migrate

`mule-lazy-migrate` is a robust CLI tool to automate migration of Mule 4 projects to a new runtime using a JSON config.

- Reads a JSON config with upgrade rules (runtime, plugin, munit versions, artifact JSON, string replacements)
- Traverses the project, updates XML (`pom.xml`), JSON (`mule-artifact.json`), and performs string replacements in source files
- Supports dry-run and file backups
- Colorized, human-friendly summary of changes at the end
- Optional Maven integration: update dependencies and build after migration

See the [main project repo](https://github.com/kchernokozinsky/mule-lazy-migrate) for more details, usage, and documentation. 