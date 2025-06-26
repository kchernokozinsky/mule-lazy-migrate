use log;
use serde_json::Value;
use std::fs;

// Placeholder for JSON operations
pub fn update_mule_artifact_json(
    path: &str,
    min_mule_version: &str,
    java_versions: &[String],
    dry_run: bool,
    backup: bool,
) {
    log::info!("Reading mule-artifact.json from {}", path);
    let json_data = fs::read_to_string(path).expect("Failed to read mule-artifact.json");
    let mut v: Value =
        serde_json::from_str(&json_data).expect("Failed to parse mule-artifact.json");

    let mut changed = false;

    // Check minMuleVersion
    let current_min_version = v["minMuleVersion"].as_str().unwrap_or("not set");
    if current_min_version != min_mule_version {
        log::info!(
            "  Updating minMuleVersion: '{}' -> '{}'",
            current_min_version,
            min_mule_version
        );
        v["minMuleVersion"] = Value::String(min_mule_version.to_string());
        changed = true;
    } else {
        log::info!("  minMuleVersion already at '{}'", min_mule_version);
    }

    // Check javaSpecificationVersions
    let new_java_versions = Value::Array(
        java_versions
            .iter()
            .map(|s| Value::String(s.clone()))
            .collect(),
    );
    let current_java_versions = &v["javaSpecificationVersions"];
    if current_java_versions != &new_java_versions {
        log::info!(
            "  Updating javaSpecificationVersions: {:?} -> {:?}",
            current_java_versions,
            new_java_versions
        );
        v["javaSpecificationVersions"] = new_java_versions;
        changed = true;
    } else {
        log::info!(
            "  javaSpecificationVersions already at {:?}",
            new_java_versions
        );
    }

    if changed {
        if backup {
            let backup_path = format!("{}.bak", path);
            fs::copy(path, &backup_path).expect("Failed to create backup");
            log::info!("Backup created: {}", backup_path);
        }
        if dry_run {
            log::info!("[DRY-RUN] Would update mule-artifact.json with the above changes");
        } else {
            log::info!("Writing updated mule-artifact.json...");
            let out = serde_json::to_string_pretty(&v).expect("Failed to serialize JSON");
            fs::write(path, out).expect("Failed to write mule-artifact.json");
            log::info!("✅ Successfully updated mule-artifact.json");
        }
    } else {
        log::info!(
            "✅ No changes needed for mule-artifact.json - all values are already up to date"
        );
    }
}

pub fn update_mule_artifact_json_summary(
    path: &str,
    min_mule_version: &str,
    java_spec_versions: &[String],
    dry_run: bool,
    backup: bool,
) -> (bool, Vec<String>) {
    let mut changed = false;
    let mut updated_fields = Vec::new();
    let mut json_data: Value =
        serde_json::from_str(&fs::read_to_string(path).expect("Failed to read mule-artifact.json"))
            .expect("Invalid JSON");

    if let Some(obj) = json_data.as_object_mut() {
        if let Some(v) = obj.get_mut("minMuleVersion") {
            if v != min_mule_version {
                updated_fields.push(format!("minMuleVersion: '{}' -> '{}'", v, min_mule_version));
                *v = Value::String(min_mule_version.to_string());
                changed = true;
            }
        }
        if let Some(v) = obj.get_mut("requiredProduct") {
            if let Some(req_obj) = v.as_object_mut() {
                if let Some(jv) = req_obj.get_mut("javaSpecificationVersions") {
                    let new_val = Value::Array(
                        java_spec_versions
                            .iter()
                            .map(|s| Value::String(s.clone()))
                            .collect(),
                    );
                    if jv != &new_val {
                        updated_fields
                            .push("requiredProduct.javaSpecificationVersions".to_string());
                        *jv = new_val;
                        changed = true;
                    }
                }
            }
        }
    }
    if changed {
        if backup {
            let backup_path = format!("{}.bak", path);
            fs::copy(path, &backup_path).expect("Failed to create backup");
        }
        if !dry_run {
            fs::write(path, serde_json::to_string_pretty(&json_data).unwrap())
                .expect("Failed to write mule-artifact.json");
        }
    }
    (changed, updated_fields)
}
