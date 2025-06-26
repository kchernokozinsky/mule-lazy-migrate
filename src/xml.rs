use log;
use regex::Regex;
use std::fs;

// Placeholder for XML operations
pub fn update_pom_xml(
    path: &str,
    runtime_version: &str,
    plugin_version: &str,
    munit_version: &str,
    dry_run: bool,
    backup: bool,
) {
    log::info!("Reading pom.xml from {path}");
    let mut xml_data = fs::read_to_string(path).expect("Failed to read pom.xml");
    let mut changed = false;

    // Helper to update only the value inside a property tag
    fn update_property_value(content: &mut String, property_name: &str, new_value: &str) -> bool {
        let pattern = format!(r#"(<{property_name}>)([^<]*)(</{property_name}>)"#);
        let re = Regex::new(&pattern).unwrap();
        let mut did_change = false;
        *content = re
            .replace_all(content, |caps: &regex::Captures| {
                let old_value = caps.get(2).map(|m| m.as_str()).unwrap_or("");
                if old_value.trim() != new_value {
                    did_change = true;
                    log::info!(
                        "  Updating property '{}': '{}' -> '{}'",
                        property_name,
                        old_value.trim(),
                        new_value
                    );
                    format!("{}{}{}", &caps[1], new_value, &caps[3])
                } else {
                    log::info!("  Property '{property_name}' already has value '{new_value}'");
                    caps[0].to_string()
                }
            })
            .to_string();
        did_change
    }

    // Update mule.version, munit.version, mule.maven.plugin.version, app.runtime in properties
    log::info!("Checking properties in pom.xml:");
    changed |= update_property_value(&mut xml_data, "mule.version", runtime_version);
    changed |= update_property_value(&mut xml_data, "munit.version", munit_version);
    changed |= update_property_value(&mut xml_data, "mule.maven.plugin.version", plugin_version);
    changed |= update_property_value(&mut xml_data, "app.runtime", runtime_version);

    if changed {
        if backup {
            let backup_path = format!("{path}.bak");
            fs::copy(path, &backup_path).expect("Failed to create backup");
            log::info!("Backup created: {backup_path}");
        }
        if dry_run {
            log::info!("[DRY-RUN] Would update pom.xml with the above changes");
        } else {
            log::info!("Writing updated pom.xml...");
            fs::write(path, xml_data).expect("Failed to write pom.xml");
            log::info!("✅ Successfully updated pom.xml");
        }
    } else {
        log::info!("✅ No changes needed for pom.xml - all values are already up to date");
    }
}

pub fn update_pom_xml_summary(
    path: &str,
    runtime_version: &str,
    plugin_version: &str,
    munit_version: &str,
    dry_run: bool,
    backup: bool,
) -> (bool, Vec<String>) {
    let mut xml_data = fs::read_to_string(path).expect("Failed to read pom.xml");
    let mut changed = false;
    let mut updated_props = Vec::new();

    fn update_property_value(
        content: &mut String,
        property_name: &str,
        new_value: &str,
        updated_props: &mut Vec<String>,
    ) -> bool {
        let pattern = format!(r#"(<{property_name}>)([^<]*)(</{property_name}>)"#);
        let re = Regex::new(&pattern).unwrap();
        let mut did_change = false;
        *content = re
            .replace_all(content, |caps: &regex::Captures| {
                let old_value = caps.get(2).map(|m| m.as_str()).unwrap_or("");
                if old_value.trim() != new_value {
                    did_change = true;
                    updated_props.push(format!(
                        "{}: '{}' -> '{}'",
                        property_name,
                        old_value.trim(),
                        new_value
                    ));
                    format!("{}{}{}", &caps[1], new_value, &caps[3])
                } else {
                    caps[0].to_string()
                }
            })
            .to_string();
        did_change
    }

    changed |= update_property_value(
        &mut xml_data,
        "mule.version",
        runtime_version,
        &mut updated_props,
    );
    changed |= update_property_value(
        &mut xml_data,
        "munit.version",
        munit_version,
        &mut updated_props,
    );
    changed |= update_property_value(
        &mut xml_data,
        "mule.maven.plugin.version",
        plugin_version,
        &mut updated_props,
    );
    changed |= update_property_value(
        &mut xml_data,
        "app.runtime",
        runtime_version,
        &mut updated_props,
    );

    if changed {
        if backup {
            let backup_path = format!("{path}.bak");
            fs::copy(path, &backup_path).expect("Failed to create backup");
        }
        if !dry_run {
            fs::write(path, xml_data).expect("Failed to write pom.xml");
        }
    }
    (changed, updated_props)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_update_pom_xml_summary_changes() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("pom.xml");
        let xml = r#"<project><properties><mule.version>4.3.0</mule.version><munit.version>3.2.0</munit.version><mule.maven.plugin.version>4.1.0</mule.maven.plugin.version><app.runtime>4.2.2</app.runtime></properties></project>"#;
        let mut file = File::create(&file_path).unwrap();
        file.write_all(xml.as_bytes()).unwrap();
        let (changed, props) = update_pom_xml_summary(
            file_path.to_str().unwrap(),
            "4.9.4",
            "4.3.1",
            "3.4.0",
            false,
            false,
        );
        assert!(changed);
        assert!(props.iter().any(|p| p.contains("mule.version")));
        assert!(props.iter().any(|p| p.contains("munit.version")));
        assert!(props
            .iter()
            .any(|p| p.contains("mule.maven.plugin.version")));
        assert!(props.iter().any(|p| p.contains("app.runtime")));
    }

    #[test]
    fn test_update_pom_xml_summary_no_change() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("pom.xml");
        let xml = r#"<project><properties><mule.version>4.9.4</mule.version><munit.version>3.4.0</munit.version><mule.maven.plugin.version>4.3.1</mule.maven.plugin.version><app.runtime>4.9.4</app.runtime></properties></project>"#;
        let mut file = File::create(&file_path).unwrap();
        file.write_all(xml.as_bytes()).unwrap();
        let (changed, props) = update_pom_xml_summary(
            file_path.to_str().unwrap(),
            "4.9.4",
            "4.3.1",
            "3.4.0",
            false,
            false,
        );
        assert!(!changed);
        assert!(props.is_empty());
    }
}
