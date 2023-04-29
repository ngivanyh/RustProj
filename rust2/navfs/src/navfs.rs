use std::fs;

fn filetype(path: &str) -> std::io::Result<()> {
	let metadata = fs::metadata(path)?;

	()
}