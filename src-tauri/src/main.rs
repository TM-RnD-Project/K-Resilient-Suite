// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod kr_ibe;
mod kr_ibi;
mod kr_peks;
mod kr_paeks;

use kr_ibe::api::*;
use kr_ibi::api::*;
use kr_peks::api::*;
use kr_paeks::api::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
             // KR-IBE
             kr_ibe_setup,
             kr_ibe_extract,
             kr_ibe_encrypt,
             kr_ibe_decrypt,
 
             // KR-IBI
             kr_ibi_setup,
             kr_ibi_extract,
             kr_ibi_sign,
             kr_ibi_verify,
 
             // KR-PEKS
             kr_peks_setup,
             kr_peks_keygen,
             kr_peks_encrypt,
             kr_peks_trapdoor,
             kr_peks_test,
 
             // KR-PAEKS
             kr_paeks_setup,
             kr_paeks_keygen,
             kr_paeks_encrypt,
             kr_paeks_trapdoor,
             kr_paeks_test
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}