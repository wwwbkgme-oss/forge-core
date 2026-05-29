//! `Plugin.toml` parser → [`PluginManifest`].

use serde::Deserialize;
use std::path::Path;
use types::{ForgeError, ForgeResult, PluginCapability, PluginManifest};

#[derive(Deserialize)]
struct ManifestFile {
    plugin:       PS,
    #[serde(default)] capabilities: CS,
    entry:        ES,
}
#[derive(Deserialize)]
struct PS { id: String, version: String, name: String, description: String }
#[derive(Default, Deserialize)]
struct CS { #[serde(default)] provides: Vec<String>, #[serde(default)] requires: Vec<String> }
#[derive(Deserialize)]
struct ES { lib: String }

/// Parse a `Plugin.toml` at `path` into a [`PluginManifest`].
///
/// The `lib` path is resolved relative to the manifest file's parent directory.
pub fn parse_manifest(path: impl AsRef<Path>) -> ForgeResult<PluginManifest> {
    let text = std::fs::read_to_string(path.as_ref())?;
    let f: ManifestFile = toml::from_str(&text)
        .map_err(|e| ForgeError::PluginLoadError(e.to_string()))?;
    let dir = path.as_ref().parent().unwrap_or(Path::new("."));
    Ok(PluginManifest {
        id:          f.plugin.id,
        version:     f.plugin.version,
        name:        f.plugin.name,
        description: f.plugin.description,
        provides:    f.capabilities.provides.iter().map(|s| PluginCapability(s.clone())).collect(),
        requires:    f.capabilities.requires.iter().map(|s| PluginCapability(s.clone())).collect(),
        lib:         dir.join(&f.entry.lib).to_string_lossy().into_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    const SAMPLE: &str = r#"
[plugin]
id = "forgefabrik.example-v1"
version = "0.1.0"
name = "Example"
description = "ref plugin"
[capabilities]
provides = ["agent"]
requires = []
[entry]
lib = "libplugin_example.so"
"#;
    #[test]
    fn parse_valid() {
        let dir  = tempfile::tempdir().unwrap();
        let p    = dir.path().join("Plugin.toml");
        std::fs::File::create(&p).unwrap().write_all(SAMPLE.as_bytes()).unwrap();
        let m = parse_manifest(&p).unwrap();
        assert_eq!(m.id, "forgefabrik.example-v1");
        assert_eq!(m.provides, vec![PluginCapability("agent".into())]);
        assert!(m.requires.is_empty());
    }
    #[test]
    fn missing_file_is_error() {
        assert!(parse_manifest("/nonexistent/Plugin.toml").is_err());
    }
}
