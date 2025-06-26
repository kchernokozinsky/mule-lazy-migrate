pub mod config;
pub mod file_ops;
pub mod json_ops;
pub mod xml;

use colored::*;
use config::MigrationConfig;
use std::path::Path;
use std::process::Command;

pub struct MigrationOptions<'a> {
    pub config_path: &'a str,
    pub project_root: &'a str,
    pub dry_run: bool,
    pub backup: bool,
    pub update_maven_deps: bool,
    pub build_mule_project: bool,
}

fn update_maven_dependencies(project_root: &str) {
    log::info!(
        "Running 'mvn versions:use-latest-releases' in {}",
        project_root
    );
    let status = Command::new("mvn")
        .arg("versions:use-latest-releases")
        .current_dir(project_root)
        .status();
    match status {
        Ok(s) if s.success() => log::info!("Maven dependencies updated to latest releases."),
        Ok(s) => log::error!("Maven exited with status: {}", s),
        Err(e) => log::error!("Failed to run Maven: {}", e),
    }
}

fn build_mule_project(project_root: &str) {
    log::info!("Running 'mvn clean install' in {}", project_root);
    let status = Command::new("mvn")
        .arg("clean")
        .arg("install")
        .current_dir(project_root)
        .status();
    match status {
        Ok(s) if s.success() => log::info!("Mule project built successfully."),
        Ok(s) => log::error!("Maven exited with status: {}", s),
        Err(e) => log::error!("Failed to run Maven: {}", e),
    }
}

fn is_mule_project(project_root: &str) -> bool {
    let pom = Path::new(project_root).join("pom.xml");
    let artifact = Path::new(project_root).join("mule-artifact.json");
    pom.exists() && artifact.exists()
}

pub fn run_migration(opts: &MigrationOptions) -> Result<(), Box<dyn std::error::Error>> {
    let mut changed_files = Vec::new();
    let mut changed_properties = Vec::new();
    let mut changed_json = Vec::new();
    let mut replacements_summary = Vec::new();
    let mut errors = Vec::new();

    log::info!("Checking if '{}' is a Mule project...", opts.project_root);
    if !is_mule_project(opts.project_root) {
        let msg = format!(
            "'{}' is not a Mule project (pom.xml or mule-artifact.json missing)",
            opts.project_root
        );
        log::error!("{}", msg);
        errors.push(msg.clone());
        print_summary(
            &changed_files,
            &changed_properties,
            &changed_json,
            &replacements_summary,
            &errors,
            opts.dry_run,
        );
        return Err(msg.into());
    }
    log::info!("Loading migration config from {}", opts.config_path);
    let config = MigrationConfig::from_file(opts.config_path)?;
    let project_root = opts.project_root;

    if opts.update_maven_deps {
        update_maven_dependencies(project_root);
    }

    if opts.build_mule_project {
        build_mule_project(project_root);
    }

    // 1. Update pom.xml
    let pom_path = Path::new(project_root).join("pom.xml");
    if pom_path.exists() {
        log::info!("Updating pom.xml at {}", pom_path.display());
        let (changed, props) = xml::update_pom_xml_summary(
            pom_path.to_str().unwrap(),
            &config.app_runtime_version,
            &config.mule_maven_plugin_version,
            &config.munit_version,
            opts.dry_run,
            opts.backup,
        );
        if changed {
            changed_files.push(pom_path.display().to_string());
            changed_properties.extend(props);
        }
    } else {
        let msg = format!("No pom.xml found at {}", pom_path.display());
        log::warn!("{}", msg);
        errors.push(msg);
    }

    // 2. Update mule-artifact.json
    let artifact_path = Path::new(project_root).join("mule-artifact.json");
    if artifact_path.exists() {
        log::info!("Updating mule-artifact.json at {}", artifact_path.display());
        let (changed, json_fields) = json_ops::update_mule_artifact_json_summary(
            artifact_path.to_str().unwrap(),
            &config.mule_artifact.min_mule_version,
            &config.mule_artifact.java_specification_versions[..],
            opts.dry_run,
            opts.backup,
        );
        if changed {
            changed_files.push(artifact_path.display().to_string());
            changed_json.extend(json_fields);
        }
    } else {
        let msg = format!("No mule-artifact.json found at {}", artifact_path.display());
        log::warn!("{}", msg);
        errors.push(msg);
    }

    // 3. Traverse and replace in source files
    let replacements_vec: Vec<(String, String)> = config
        .replacements
        .iter()
        .map(|r| (r.from.clone(), r.to.clone()))
        .collect();
    let rep_summary = file_ops::traverse_and_replace_summary(
        project_root,
        &replacements_vec,
        opts.dry_run,
        opts.backup,
    );
    replacements_summary.extend(rep_summary);

    print_summary(
        &changed_files,
        &changed_properties,
        &changed_json,
        &replacements_summary,
        &errors,
        opts.dry_run,
    );
    Ok(())
}

fn print_summary(
    changed_files: &[String],
    changed_properties: &[String],
    changed_json: &[String],
    replacements_summary: &[String],
    errors: &[String],
    dry_run: bool,
) {
    println!(
        "\n{}",
        "================ MIGRATION SUMMARY ================"
            .bold()
            .blue()
    );
    if dry_run {
        println!(
            "{}",
            "[DRY-RUN] No files were actually changed".bold().blue()
        );
    }
    if !changed_files.is_empty() {
        println!("{}", "Changed files:".green().bold());
        for file in changed_files {
            println!("  {}", file.green());
        }
    }
    if !changed_properties.is_empty() {
        println!("{}", "Updated properties:".green().bold());
        for prop in changed_properties {
            println!("  {}", prop.green());
        }
    }
    if !changed_json.is_empty() {
        println!("{}", "Updated JSON fields:".green().bold());
        for field in changed_json {
            println!("  {}", field.green());
        }
    }
    if !replacements_summary.is_empty() {
        println!("{}", "String replacements:".yellow().bold());
        for rep in replacements_summary {
            println!("  {}", rep.yellow());
        }
    }
    if !errors.is_empty() {
        println!("{}", "Warnings/Errors:".red().bold());
        for err in errors {
            println!("  {}", err.red());
        }
    }
    if changed_files.is_empty()
        && changed_properties.is_empty()
        && changed_json.is_empty()
        && replacements_summary.is_empty()
        && errors.is_empty()
    {
        println!(
            "{}",
            "No changes were needed. Project is up to date!"
                .blue()
                .bold()
        );
    }
    println!(
        "{}",
        "=================================================="
            .bold()
            .blue()
    );
}
