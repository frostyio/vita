pub mod macros;
use std::path::PathBuf;

pub fn out_dir() -> PathBuf {
	let mut dir = std::env::current_dir().unwrap();
	dir.pop();
	dir.pop();
	dir.join("out")
}

pub fn luatests() -> PathBuf {
	let mut dir = std::env::current_dir().unwrap();
	dir.pop();
	dir.pop();
	dir.join("luatests")
}
