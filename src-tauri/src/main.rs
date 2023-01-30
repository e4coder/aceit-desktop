#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::process::Command;
use std::str::from_utf8;

#[tauri::command]
fn encrypt(text: &str, key: &str) -> String {
    let output = if cfg!(target_os = "windows") {
        Command::new("powershell")
                .args(["/C", &format!("aceit encrypt --text=\"{}\" --key=\"{}\"", text, key)])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(&format!("aceit encrypt --text=\"{}\" --key=\"{}\"", text, key))
                .output()
                .expect("failed to execute process")
    };

    let hello = output.stdout;
    let out = String::from(from_utf8(&hello).expect("Invalid Utf-8"));
    println!("OUT PUT IS THIS {}", out);
    out
}

#[tauri::command]
fn decrypt(text: &str, key: &str) -> String {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(["/C", &format!("aceit decrypt --text=\"{}\" --key=\"{}\"", text, key)])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(&format!("aceit decrypt --text=\"{}\" --key=\"{}\"", text, key))
                .output()
                .expect("failed to execute process")
    };

    let hello = output.stdout;
    let out = String::from(from_utf8(&hello).expect("Invalid Utf-8"));
    println!("OUT PUT IS THIS {}", out);
    out
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, encrypt, decrypt])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
