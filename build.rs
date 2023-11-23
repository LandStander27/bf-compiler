
fn copy_dir_all(src: impl AsRef<std::path::Path>, dest: impl AsRef<std::path::Path>) {
	std::fs::create_dir_all(&dest).unwrap();
	for entry in std::fs::read_dir(src).unwrap() {
		let entry = entry.unwrap();
		if entry.metadata().unwrap().is_dir() {
			copy_dir_all(entry.path(), dest.as_ref().join(entry.file_name()));
		} else {
			std::fs::copy(entry.path(), dest.as_ref().join(entry.file_name())).unwrap();
		}
	}
}

fn main() {
	let out = std::env::var("PROFILE").unwrap();
	let out = std::path::PathBuf::from(format!("target/{}/tcc", out));

	if out.exists() {
		std::fs::remove_dir_all(&out).unwrap();
	}
	std::fs::create_dir(&out).unwrap();
	copy_dir_all("tcc", out);

}