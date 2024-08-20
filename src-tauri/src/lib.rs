use nothing::anc::AncMode;
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

#[tauri::command]
async fn set_anc_mode_off(state: tauri::State<'_, Ear2>) -> Result<(), ()> {
    let k = state.set_anc_mode(AncMode::Off).await;
    println!("set_anc_mode_off: {:?}", k);
    Ok(())
}

#[tauri::command]
async fn set_anc_mode_high(state: tauri::State<'_, Ear2>) -> Result<(), ()> {
    let k = state.set_anc_mode(AncMode::High).await;
    println!("set_anc_mode_high: {:?}", k);
    Ok(())
}

#[tauri::command]
async fn set_anc_mode_mid(state: tauri::State<'_, Ear2>) -> Result<(), ()> {
    let k = state.set_anc_mode(AncMode::Mid).await;
    println!("set_anc_mode_mid: {:?}", k);
    Ok(())
}

#[tauri::command]
async fn set_anc_mode_low(state: tauri::State<'_, Ear2>) -> Result<(), ()> {
    let k = state.set_anc_mode(AncMode::Low).await;
    println!("set_anc_mode_low: {:?}", k);
    Ok(())
}

#[tauri::command]
async fn set_anc_mode_adaptive(state: tauri::State<'_, Ear2>) -> Result<(), ()> {
    let k = state.set_anc_mode(AncMode::Adaptive).await;
    println!("set_anc_mode_adaptive: {:?}", k);
    Ok(())
}

#[tauri::command]
async fn set_anc_mode_transparency(state: tauri::State<'_, Ear2>) -> Result<(), ()> {
    let k = state.set_anc_mode(AncMode::Transparency).await;
    println!("set_anc_mode_transparency: {:?}", k);
    Ok(())
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
            get_serial_number,
            set_anc_mode_off,
            set_anc_mode_high,
            set_anc_mode_mid,
            set_anc_mode_low,
            set_anc_mode_adaptive,
            set_anc_mode_transparency
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
