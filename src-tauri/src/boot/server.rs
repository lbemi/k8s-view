use super::config::load_k8s_config;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
const KUBENRETE_CONFIG_PATH: &str = ".kube/config";
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let k8s_config = load_k8s_config(KUBENRETE_CONFIG_PATH).unwrap();

    println!("{:?}", k8s_config);
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
