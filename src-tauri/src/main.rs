#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![create_window])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
async fn create_window(app: tauri::AppHandle, label: String, figma_user: String, full_screen: bool, x: f64, y: f64) {
  std::thread::spawn(move || {
    tauri::WindowBuilder::new(&app, label, tauri::WindowUrl::External("https://figma.com/mirror".parse().unwrap()))
    .fullscreen(full_screen)
    .position(x, y)
    .build()
    .unwrap();
    });
}
