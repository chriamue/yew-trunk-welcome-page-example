pub const BASE_PATH: &str = env!("BASE_PATH");

pub fn full_path(path: &str) -> String {
    if path == "/" {
        BASE_PATH.to_string()
    } else {
        format!("{}{}", BASE_PATH, path)
    }
}
