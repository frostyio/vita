mod common;
use crate::shared::{luatests, out_dir};
use anyhow::Result;
pub use common::inst;
use std::{
	io::Write,
	process::{Command, Stdio},
};

pub fn compile(script_name: &str) -> Result<String> {
	let script = luatests().join(script_name);
	let output = Command::new("/home/frosty/Documents/lua-5.1.5/src/luac")
		.arg("-o")
		.arg("-")
		.arg(script)
		.output()?;

	Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn hexyl(data: &str) -> Result<String> {
	let mut proc = Command::new("hexyl")
		.arg("-n")
		.arg(data.len().to_string())
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()?;

	proc.stdin.as_mut().unwrap().write_all(data.as_bytes())?;

	let output = proc.wait_with_output()?;
	Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[test]
fn test_compile() {
	let compiled = compile("numerical_for.lua").unwrap();
	// hexyl(&compiled);
	println!("{}", hexyl(&compiled).unwrap());
}
