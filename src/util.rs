use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;
use cambia_core::handler::parse_log_bytes;

#[cfg(feature = "python")]
use serde_json::Value;

// This function is only used in binary mode
#[cfg(all(not(feature = "python"), not(target_os = "unknown")))]
pub fn parse_file(filepath: &str, args: crate::Args) {
	let mut raw: Vec<u8> = Vec::new();

	let mut fh = OpenOptions::new().read(true).open(filepath).expect(
		"Could not open file",
	);

	fh.read_to_end(&mut raw).expect(
		"Could not read file"
	);

	let parsed = match parse_log_bytes(Vec::new(), &raw) {
		Ok(parsed) => parsed,
		Err(_) => return,
	};

	if let Ok(parsed) = parse_log_bytes(Vec::new(), &raw) {
		println!("{}", serde_json::to_string(&parsed).unwrap());
	}

	if let Some(save_logs) = args.save_logs {
		save_rip_log(save_logs, &parsed.id, &raw);
	}
}

pub fn save_rip_log(root_path: PathBuf, id: &[u8], log_raw: &[u8]) {
	if let Err(e) = std::fs::create_dir_all(&root_path) {
		tracing::error!("Error creating directory: {}", e);
		return;
	}

	let file_path = root_path.join(hex::encode(id)).with_extension("log");

	if !file_path.exists() {
		match std::fs::File::create(&file_path).and_then(|mut file| std::io::Write::write_all(&mut file, log_raw)) {
			Ok(_) => (),
			Err(e) => tracing::error!("Error writing file: {}", e),
		}
	}
}
/// Parse a file for Python bindings
#[cfg(feature = "python")]
pub fn parse_file_for_python(filepath: &str) -> Result<Value, String> {
    let mut raw: Vec<u8> = Vec::new();

    let mut fh = OpenOptions::new().read(true).open(filepath)
        .map_err(|e| format!("Could not open file: {}", e))?;
    fh.read_to_end(&mut raw)
        .map_err(|e| format!("Could not read file: {}", e))?;

    let parsed = parse_log_bytes(Vec::new(), &raw)
        .map_err(|e| format!("Could not parse log: {:?}", e))?;
    let json_value = serde_json::to_value(&parsed)
        .map_err(|e| format!("Could not serialize to JSON: {}", e))?;
    
    Ok(json_value)
}

/// Parse content from string for Python bindings
#[cfg(feature = "python")]
pub fn parse_content_for_python(content: &str) -> Result<Value, String> {
    let raw = content.as_bytes().to_vec();
    let parsed = parse_log_bytes(Vec::new(), &raw)
        .map_err(|e| format!("Could not parse log: {:?}", e))?;
    let json_value = serde_json::to_value(&parsed)
        .map_err(|e| format!("Could not serialize to JSON: {}", e))?;
    
    Ok(json_value)
}