use clap::Parser;
use mule_lazy_migrate::{run_migration, MigrationOptions};

#[derive(Parser)]
#[command(name = "mule-lazy-migrate")]
#[command(about = "Migrate Mule 4 projects to a new runtime using a JSON config. The summary at the end is colorized for clarity.", long_about = None)]
struct Cli {
    /// Path to the JSON config file
    #[arg(short, long)]
    config: String,

    /// Perform a dry run without making changes
    #[arg(long)]
    dry_run: bool,

    /// Backup files before modifying (default: false)
    #[arg(long, default_value_t = false)]
    backup: bool,

    /// Path to the Mule project root (default: current directory)
    #[arg(short, long, default_value = ".")]
    project: String,

    /// Also update all Maven dependencies to their latest release versions
    #[arg(short = 'u', long)]
    update_maven_deps: bool,

    /// Build the Mule project with 'mvn clean install' after migration
    #[arg(short = 'b', long)]
    build_mule_project: bool,
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let cli = Cli::parse();
    let opts = MigrationOptions {
        config_path: &cli.config,
        project_root: &cli.project,
        dry_run: cli.dry_run,
        backup: cli.backup,
        update_maven_deps: cli.update_maven_deps,
        build_mule_project: cli.build_mule_project,
    };
    if let Err(e) = run_migration(&opts) {
        eprintln!("Migration failed: {}", e);
        std::process::exit(1);
    }
}
