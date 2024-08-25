use napi_derive::napi;
use std::path::PathBuf;

#[napi(object)]
#[derive(Debug)]
pub struct PackageInfo {
  pub name: String,
  pub version: String,
  pub description: String,
  pub keywords: Vec<String>,
  #[napi(js_name = "path")]
  pub path_str: String,
  pub homepage: String,
  pub license: String,
  pub author: String
}

impl PackageInfo {
  pub fn new(
    name: String,
    version: String,
    description: String,
    keywords: Vec<String>,
    path: PathBuf,
    homepage: String,
    license: String,
    author: String
  ) -> Self {
    PackageInfo {
      name,
      version,
      description,
      keywords,
      path_str: path.to_string_lossy().into_owned(),
      homepage,
      license,
      author
    }
  }
}