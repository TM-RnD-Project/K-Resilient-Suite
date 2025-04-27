use super::params::Params;
use super::private_key::PrivateKey;
use super::ciphertext::Ciphertext;
use super::plaintext::Plaintext;
use super::main as kribe_core;

use std::sync::Mutex;
use once_cell::sync::Lazy;

static PARAMS: Lazy<Mutex<Option<Params>>> = Lazy::new(|| Mutex::new(None));
static SK: Lazy<Mutex<Option<PrivateKey>>> = Lazy::new(|| Mutex::new(None));
static CT: Lazy<Mutex<Option<Ciphertext>>> = Lazy::new(|| Mutex::new(None));
static PT: Lazy<Mutex<Option<Plaintext>>> = Lazy::new(|| Mutex::new(None));

#[tauri::command]
pub fn setup_command() -> String {
    println!("Reached setup_command");
    let mut params = Params::new();
    kribe_core::setup(&mut params);
    *PARAMS.lock().unwrap() = Some(params);
    "Setup complete.".into()
}

#[tauri::command]
pub fn extract_command(id: String) -> String {
    let id_bytes = id.into_bytes();

    let maybe_params = PARAMS.lock().unwrap();
    if maybe_params.is_none() {
        return "Params not initialized. Please run setup first.".into();
    }

    let params = maybe_params.as_ref().unwrap();
    let mut sk = PrivateKey::new();

    kribe_core::extract(&params, &mut sk, &id_bytes);

    let output = format!("Private key extracted for ID.\n");
    *SK.lock().unwrap() = Some(sk);
    output
}

#[tauri::command]
pub fn encrypt_command(id: String, plaintext: String) -> String {
    let id_bytes = id.into_bytes();
    let msg_bytes = plaintext.into_bytes();

    let maybe_params = PARAMS.lock().unwrap();
    if maybe_params.is_none() {
        return "Params not initialized. Please run setup first.".into();
    }

    let params = maybe_params.as_ref().unwrap();
    let mut ciphertext = Ciphertext::new();

    kribe_core::encryption(&params, &mut ciphertext, &id_bytes, &msg_bytes);

    *CT.lock().unwrap() = Some(ciphertext);
    "Encryption completed successfully.".into()
}

#[tauri::command]
pub fn decrypt_command() -> String {
    // Step 1: Lock and extract params, sk, and ct one-by-one

    let mut pt = Plaintext::new();

    {
        let mut pt_lock = PT.lock().unwrap();

        let params_guard = PARAMS.lock().unwrap();
        let sk_guard = SK.lock().unwrap();
        let mut ct_guard = CT.lock().unwrap();

        // Validate
        if params_guard.is_none() || sk_guard.is_none() || ct_guard.is_none() {
            return "Missing data. Please run setup, extract, and encrypt first.".into();
        }

        // Use the references directly (without cloning)
        let params = params_guard.as_ref().unwrap();
        let sk = sk_guard.as_ref().unwrap();
        let ct = ct_guard.as_mut().unwrap(); // mutable borrow is allowed now

        kribe_core::decryption(params, sk, ct, &mut pt);

        *pt_lock = Some(pt);
    }

    // Output result after lock is released
    let pt_read = PT.lock().unwrap();
    let result = pt_read.as_ref().unwrap().to_string();
    format!("Decrypted message:\n{}", result)
}


