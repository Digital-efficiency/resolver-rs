use std::path::{Path, PathBuf};
use std::fs;
use serde_json::Value;
use crate::package_info::PackageInfo;

pub struct Resolver {
  base_dir: PathBuf,
}

impl Resolver {
  pub fn new(base_dir: PathBuf) -> Self {
    Resolver { base_dir }
  }

  pub fn resolve(&self, package_name: &str) -> Option<PackageInfo> {
    let mut current_dir = self.base_dir.clone();
    loop {
      let node_modules = current_dir.join("node_modules");
      let package_dir = node_modules.join(package_name);
      if package_dir.is_dir() {
        return self.get_package_info(&package_dir, package_name);
      }
      if package_name.starts_with('@') {
        let parts: Vec<&str> = package_name.split('/').collect();
        if parts.len() == 2 {
          let org_package_dir = node_modules.join(parts[0]).join(parts[1]);
          if org_package_dir.is_dir() {
            return self.get_package_info(&org_package_dir, package_name);
          }
        }
      }
      if let Some(parent) = current_dir.parent() {
        current_dir = parent.to_path_buf();
      } else {
        return None;
      }
    }
  }

  fn get_package_info(&self, package_dir: &Path, package_name: &str) -> Option<PackageInfo> {
    let package_json_path = package_dir.join("package.json");
    if package_json_path.is_file() {
      if let Ok(content) = fs::read_to_string(&package_json_path) {
        if let Ok(json) = serde_json::from_str::<Value>(&content) {
          return Some(PackageInfo::new(
            package_name.to_string(),
            json["version"].as_str().unwrap_or("unknown").to_string(),
            json["description"].as_str().unwrap_or("").to_string(),
            json["keywords"].as_array()
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_else(Vec::new),
            package_dir.to_path_buf(),
            json["homepage"].as_str().unwrap_or("").to_string(),
            json["license"].as_str().unwrap_or("").to_string(),
            json["author"].as_str().unwrap_or("").to_string()
          ));
        }
      }
    }
    None
  }
}