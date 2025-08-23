use walkdir::WalkDir;


pub fn discover_files(root: &str) -> Vec<String> {
    let all_paths = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| {
            if let Some(ext) = e.path().extension() {
                ext == "py"
            } else {
                false
            }
        })
        .map(|e| e.path().to_string_lossy().to_string())
        .collect::<Vec<String>>();

    all_paths
}