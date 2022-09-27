use mlua::{Lua, DeserializeOptions, LuaSerdeExt};
use serde::{Deserialize, Serialize};

use crate as krait;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Manifest {
	pub name: String,
	pub latest_commit: String,
	pub last_update: String,
	pub packages: Vec<ManifestPackage>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ManifestPackage {
	pub version: String,
	pub commit: String,
	pub contents: Vec<ManifestPackageContent>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ManifestPackageContent {
	pub name: String,
	pub path: String,
	pub sha256: String,
	pub url: String
}

impl Manifest {
	pub fn parse(s: String) -> Self {
		let lua = Lua::new();
		let globals = lua.globals();

		let krait_table = lua.create_table().expect("Failed to create krait table");
		let manifest_table = lua.create_table().expect("Failed to create manifest table");

		krait_table
			.set("manifest", manifest_table)
			.expect("Failed to set manifest table");

		globals
			.set("krait", krait_table)
			.expect("Failed to set krait table");

		// load the manifest
		let manifest = lua.load(&s).eval::<mlua::Table>().expect("Failed to load manifest");

		let options = DeserializeOptions::new()
			.deny_unsupported_types(false)
			.deny_recursive_tables(false);

		let manifest: Manifest = match lua.from_value_with(mlua::Value::Table(manifest), options) {
			Ok(m) => m,
			Err(e) => {
				eprintln!("Error parsing manifest: {}", e);
				krait::exit!(1);
			}
		};

		dbg!(&manifest);

		manifest
		
	}
}