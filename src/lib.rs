#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod package_info;
mod resolver;

pub use package_info::PackageInfo;
pub use resolver::Resolver;

#[napi]
pub fn resolve(package_name: String) -> Option<PackageInfo> {
  let base_dir = std::env::current_dir().unwrap();
  let resolver = Resolver::new(base_dir);
  resolver.resolve(&package_name)
}