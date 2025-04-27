use tauri::command;

use crate::kr_paeks::params::Params;
use crate::kr_paeks::private_key::PrivateKey;
use crate::kr_paeks::public_key::PublicKey;
use crate::kr_paeks::ciphertext::Ciphertext;
use crate::kr_paeks::trapdoor::Trapdoor;
use super::main as krpaeks_core;

static mut PARAMS: Option<Params> = None;
static mut SENDER_SK: Option<PrivateKey> = None;
static mut SENDER_PK: Option<PublicKey> = None;
static mut RECEIVER_SK: Option<PrivateKey> = None;
static mut RECEIVER_PK: Option<PublicKey> = None;
static mut CIPHERTEXT: Option<Ciphertext> = None;
static mut TRAPDOOR: Option<Trapdoor> = None;

#[command]
pub fn kr_paeks_setup() -> String {
    let mut params = Params::new();
    krpaeks_core::setup(&mut params);
    unsafe {
        PARAMS = Some(params);
    }
    "✅ KR-PAEKS Setup complete!".into()
}

#[command]
pub fn kr_paeks_keygen() -> String {
    unsafe {
        if let Some(ref params) = PARAMS {
            let mut sender_pk = PublicKey::new();
            let mut sender_sk = PrivateKey::new();
            let mut receiver_pk = PublicKey::new();
            let mut receiver_sk = PrivateKey::new();
            krpaeks_core::keygen(params, &mut sender_pk, &mut sender_sk);
            krpaeks_core::keygen(params, &mut receiver_pk, &mut receiver_sk);

            SENDER_PK = Some(sender_pk);
            SENDER_SK = Some(sender_sk);
            RECEIVER_PK = Some(receiver_pk);
            RECEIVER_SK = Some(receiver_sk);

            "✅ KR-PAEKS Keygen complete!".into()
        } else {
            "❌ Error: KR-PAEKS setup not done yet!".into()
        }
    }
}

#[command]
pub fn kr_paeks_encrypt(keyword: String) -> String {
    unsafe {
        if let (Some(ref params), Some(ref receiver_pk), Some(ref sender_sk)) = (PARAMS.as_ref(), RECEIVER_PK.as_ref(), SENDER_SK.as_ref()) {
            let keyword_big = krpaeks_core::hash_to_big(&keyword);
            let ct = krpaeks_core::encrypt(params, receiver_pk, sender_sk, &keyword_big);
            CIPHERTEXT = Some(ct);
            "✅ KR-PAEKS Encryption complete!".into()
        } else {
            "❌ Error: Keygen not done yet!".into()
        }
    }
}

#[command]
pub fn kr_paeks_trapdoor(keyword: String) -> String {
    unsafe {
        if let (Some(ref params), Some(ref sender_pk), Some(ref receiver_sk)) = (PARAMS.as_ref(), SENDER_PK.as_ref(), RECEIVER_SK.as_ref()) {
            let keyword_big = krpaeks_core::hash_to_big(&keyword);
            let td = krpaeks_core::trapdoor(params, sender_pk, receiver_sk, &keyword_big);
            TRAPDOOR = Some(td);
            "✅ KR-PAEKS Trapdoor generation complete!".into()
        } else {
            "❌ Error: Keygen not done yet!".into()
        }
    }
}

#[command]
pub fn kr_paeks_test() -> String {
    unsafe {
        if let (Some(ref ct), Some(ref td)) = (CIPHERTEXT.as_ref(), TRAPDOOR.as_ref()) {
            if krpaeks_core::test(ct, td) {
                "✅ KR-PAEKS Test successful!".into()
            } else {
                "❌ KR-PAEKS Test failed!".into()
            }
        } else {
            "❌ Error: Need to run encryption and trapdoor first!".into()
        }
    }
}
