use crate::config::ReplacementRule;
use log;
use std::fs;
use std::io::{Read, Write};
use walkdir::WalkDir;

const FILE_EXTENSIONS: &[&str] = &["xml", "dwl"]; // Extend as needed

pub fn traverse_and_replace(
    root: &str,
    replacements: &[ReplacementRule],
    dry_run: bool,
    backup: bool,
) {
    log::info!("🔍 Scanning for files with extensions: {FILE_EXTENSIONS:?}");
    log::info!("📝 Replacement rules to apply:");
    for (i, rule) in replacements.iter().enumerate() {
        log::info!("  {}. '{}' -> '{}'", i + 1, rule.from, rule.to);
    }

    let mut files_processed = 0;
    let mut files_updated = 0;

    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if FILE_EXTENSIONS.contains(&ext) {
                    files_processed += 1;
                    log::info!("📄 Processing: {}", path.display());

                    let mut content = String::new();
                    if let Ok(mut file) = fs::File::open(path) {
                        if file.read_to_string(&mut content).is_ok() {
                            let mut new_content = content.clone();
                            let mut replacements_made = 0;

                            for rule in replacements {
                                let count = new_content.matches(&rule.from).count();
                                if count > 0 {
                                    log::info!(
                                        "    🔄 Replacing '{}' with '{}' ({} occurrences)",
                                        rule.from,
                                        rule.to,
                                        count
                                    );
                                    new_content = new_content.replace(&rule.from, &rule.to);
                                    replacements_made += count;
                                }
                            }

                            if new_content != content {
                                files_updated += 1;
                                if backup {
                                    let backup_path = path.with_extension(format!("{ext}.bak"));
                                    fs::copy(path, &backup_path).expect("Failed to create backup");
                                    log::info!("    💾 Backup created: {}", backup_path.display());
                                }
                                if dry_run {
                                    log::info!(
                                        "    [DRY-RUN] Would update {} ({} replacements)",
                                        path.display(),
                                        replacements_made
                                    );
                                } else {
                                    let mut file =
                                        fs::File::create(path).expect("Failed to write file");
                                    file.write_all(new_content.as_bytes())
                                        .expect("Failed to write file");
                                    log::info!(
                                        "    ✅ Updated {} ({} replacements)",
                                        path.display(),
                                        replacements_made
                                    );
                                }
                            } else {
                                log::info!("    ✅ No changes needed");
                            }
                        }
                    }
                }
            }
        }
    }

    log::info!("📊 Summary: Processed {files_processed} files, updated {files_updated} files");
}

pub fn traverse_and_replace_summary(
    root: &str,
    replacements: &Vec<(String, String)>,
    dry_run: bool,
    backup: bool,
) -> Vec<String> {
    let mut summary = Vec::new();
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if [
                "xml",
                "yaml",
                "yml",
                "properties",
                "txt",
                "java",
                "groovy",
                "json",
            ]
            .contains(&ext)
            {
                let content = fs::read_to_string(path);
                if let Ok(mut content) = content {
                    let mut changed = false;
                    for (from, to) in replacements {
                        if content.contains(from) {
                            summary.push(format!("{}: '{}' -> '{}'", path.display(), from, to));
                            content = content.replace(from, to);
                            changed = true;
                        }
                    }
                    if changed {
                        if backup {
                            let backup_path = format!("{}.bak", path.display());
                            fs::copy(path, &backup_path).ok();
                        }
                        if !dry_run {
                            fs::write(path, content).ok();
                        }
                    }
                }
            }
        }
    }
    summary
}
