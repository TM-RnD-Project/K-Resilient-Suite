use tauri::command;

use crate::kr_peks::params::Params;
use crate::kr_peks::private_key::PrivateKey;
use crate::kr_peks::public_key::PublicKey;
use crate::kr_peks::ciphertext::Ciphertext;
use crate::kr_peks::trapdoor::Trapdoor;
use crate::kr_peks::utils::*;
use super::main as krpeks_core;

static mut PARAMS: Option<Params> = None;
static mut SK: Option<PrivateKey> = None;
static mut PK: Option<PublicKey> = None;
static mut CIPHERTEXT: Option<Ciphertext> = None;
static mut TRAPDOOR: Option<Trapdoor> = None;

#[command]
pub fn kr_peks_setup() -> String {
    let mut params = Params::new();
    krpeks_core::setup(&mut params);
    unsafe {
        PARAMS = Some(params);
    }
    "✅ KR-PEKS Setup complete!".into()
}

#[command]
pub fn kr_peks_extract(id: String) -> String {
    unsafe {
        if let Some(ref params) = PARAMS {
            let mut pk = PublicKey::new();
            let mut sk = PrivateKey::new();
            krpeks_core::keygen(params, &mut pk, &mut sk);

            PK = Some(pk);
            SK = Some(sk);

            "✅ KR-PEKS Key generation complete!".into()
        } else {
            "❌ Error: KR-PEKS Setup not done yet!".into()
        }
    }
}


#[command]
pub fn kr_peks_encrypt(id: String, plaintext: String) -> String {
    unsafe {
        if let (Some(ref params), Some(ref pk)) = (PARAMS.as_ref(), PK.as_ref()) {
            // Join id + plaintext as the keyword
            let keyword = format!("{}{}", id, plaintext);
            let keyword_bytes = string_to_bytes(&keyword);
            if let Some(ct) = krpeks_core::peks(params, pk, &keyword_bytes) {
                CIPHERTEXT = Some(ct);
                "✅ KR-PEKS Encryption complete!".into()
            } else {
                "❌ KR-PEKS Encryption failed!".into()
            }
        } else {
            "❌ Error: KR-PEKS Extract not done yet!".into()
        }
    }
}

#[command]
pub fn kr_peks_trapdoor(id: String) -> String {
    unsafe {
        if let (Some(ref params), Some(ref sk)) = (PARAMS.as_ref(), SK.as_ref()) {
            let id_bytes = string_to_bytes(&id);
            let td = krpeks_core::trapdoor(params, sk, &id_bytes);
            TRAPDOOR = Some(td);
            "✅ KR-PEKS Trapdoor generation complete!".into()
        } else {
            "❌ Error: KR-PEKS Extract not done yet!".into()
        }
    }
}

#[command]
pub fn kr_peks_test() -> String {
    unsafe {
        if let (Some(ref ct), Some(ref td)) = (CIPHERTEXT.as_ref(), TRAPDOOR.as_ref()) {
            if krpeks_core::test(ct, td) {
                "✅ KR-PEKS Test successful!".into()
            } else {
                "❌ KR-PEKS Test failed.".into()
            }
        } else {
            "❌ Error: Need encryption and trapdoor first!".into()
        }
    }
}
