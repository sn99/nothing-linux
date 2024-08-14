use nothing::nothing_ear_2::Ear2;
use nothing::Nothing;

#[tauri::command]
async fn get_firmware_version(state: tauri::State<'_, Ear2>) -> Result<String, ()> {
    let firmware_version = state.get_firmware_version().await;
    println!("{}", firmware_version);
    Ok(firmware_version)
}

#[tauri::command]
async fn get_address(state: tauri::State<'_, Ear2>) -> Result<String, ()> {
    let address = state.get_address().await;
    println!("{}", address);
    Ok(address)
}

#[tauri::command]
async fn get_serial_number(state: tauri::State<'_, Ear2>) -> Result<String, ()> {
    let serial_number = state.get_serial_number().await;
    println!("{}", serial_number);
    Ok(serial_number)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let ear = Ear2::new().await.unwrap();

    tauri::Builder::default()
        .manage(ear)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_firmware_version,
            get_address,
            get_serial_number
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
