use tauri::command;

use super::params::Params;
use super::main as kribi_core;

static mut PARAMS: Option<Params> = None;

#[command]
pub fn kr_ibi_setup() -> String {
    let mut params = Params::new();
    kribi_core::setup(&mut params);
    unsafe {
        PARAMS = Some(params);
    }
    "✅ KR-IBI setup complete".into()
}

#[command]
pub fn kr_ibi_extract(id: String) -> String {
    unsafe {
        if let Some(ref params) = PARAMS {
            let id_bytes = kribi_core::string_to_bytes(&id);
            let (f1, f2) = kribi_core::extract(params, &id_bytes);
            format!("Extracted KR-IBI keys:\nf(ID1): {}\nf(ID2): {}", kribi_core::big_to_hex(&f1), kribi_core::big_to_hex(&f2))
        } else {
            "❌ Error: KR-IBI setup not done yet!".into()
        }
    }
}

#[command]
pub fn kr_ibi_sign(id: String) -> String {
    unsafe {
        if let Some(ref params) = PARAMS {
            let id_bytes = kribi_core::string_to_bytes(&id);
            let (fID1, fID2) = kribi_core::extract(params, &id_bytes);
            let mut rng = kribi_core::gen_seed();
            let (g_r, r) = kribi_core::commit(params, &mut rng);
            let (c1, c2) = kribi_core::challenge(params, &mut rng);
            let (s1, s2) = kribi_core::respond(&r, &(c1, c2), &(fID1, fID2), params.get_order());
            format!("KR-IBI Signature generated:\nCommit: {:?}\nChallenge: ({}, {})\nResponse: ({}, {})",
                (kribi_core::ecp_to_hex(&g_r.0), kribi_core::ecp_to_hex(&g_r.1)),
                kribi_core::big_to_hex(&c1),
                kribi_core::big_to_hex(&c2),
                kribi_core::big_to_hex(&s1),
                kribi_core::big_to_hex(&s2))
        } else {
            "❌ Error: KR-IBI setup not done yet!".into()
        }
    }
}

#[command]
pub fn kr_ibi_verify(id: String) -> String {
    unsafe {
        if let Some(ref params) = PARAMS {
            let id_bytes = kribi_core::string_to_bytes(&id);
            let mut rng = kribi_core::gen_seed();
            let (g_r, r) = kribi_core::commit(params, &mut rng);
            let (c1, c2) = kribi_core::challenge(params, &mut rng);
            let (s1, s2) = kribi_core::respond(&r, &(c1, c2), &(kribi_core::extract(params, &id_bytes)), params.get_order());
            let valid = kribi_core::verify(params, &g_r, &(s1, s2), &(c1, c2), &id_bytes);
            if valid {
                "✅ KR-IBI Verification successful!".into()
            } else {
                "❌ KR-IBI Verification failed.".into()
            }
        } else {
            "❌ Error: KR-IBI setup not done yet!".into()
        }
    }
}
