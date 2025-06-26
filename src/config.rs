use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct MigrationConfig {
    pub app_runtime_version: String,
    pub mule_maven_plugin_version: String,
    pub munit_version: String,
    pub mule_artifact: MuleArtifactConfig,
    pub replacements: Vec<ReplacementRule>,
}

#[derive(Debug, Deserialize)]
pub struct MuleArtifactConfig {
    pub min_mule_version: String,
    pub java_specification_versions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReplacementRule {
    pub from: String,
    pub to: String,
}

impl MigrationConfig {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(path)?;
        let config: MigrationConfig = serde_json::from_str(&data)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_migration_config_from_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_config.json");
        let json = r#"{
            "app_runtime_version": "4.9.4",
            "mule_maven_plugin_version": "4.3.1",
            "munit_version": "3.4.0",
            "mule_artifact": {
                "min_mule_version": "4.9.0",
                "java_specification_versions": ["17"]
            },
            "replacements": [
                {"from": "foo", "to": "bar"}
            ]
        }"#;
        let mut file = File::create(&file_path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        let config = MigrationConfig::from_file(&file_path).unwrap();
        assert_eq!(config.app_runtime_version, "4.9.4");
        assert_eq!(config.mule_maven_plugin_version, "4.3.1");
        assert_eq!(config.munit_version, "3.4.0");
        assert_eq!(config.mule_artifact.min_mule_version, "4.9.0");
        assert_eq!(config.mule_artifact.java_specification_versions, vec!["17"]);
        assert_eq!(config.replacements[0].from, "foo");
        assert_eq!(config.replacements[0].to, "bar");
    }
}
