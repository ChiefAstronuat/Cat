pub mod audio;
pub mod brain;
pub mod governor;
pub mod identity;
pub mod interaction;
pub mod overlay;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("Virtual Desktop Cat Booting...");
    
    // 각종 Agent 초기화
    overlay::init_overlay();
    interaction::listen_interactions();
    brain::init_brain();
    identity::load_identity();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
