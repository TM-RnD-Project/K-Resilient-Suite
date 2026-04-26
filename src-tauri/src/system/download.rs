use crate::system::state::APP_STATE;
use crate::kr_ibe::{main as kribe_core, plaintext::Plaintext, private_key::PrivateKey, params::Params};

pub fn download(user: &str, index: usize) -> String {
    // -------- Step 1: safe state access --------
    let (params, sk) = {
        let state = APP_STATE.lock().unwrap();

        let params = match &state.ibe_params {
            Some(p) => p.clone(),
            None => return "Error: system not initialized".to_string(),
        };

        let sk = match state.users.get(user) {
            Some(k) => k.clone(),
            None => return "Error: user not registered".to_string(),
        };

        (params, sk)
    };

    // -------- Step 2: safe index check --------
    let mut ct = {
        let state = APP_STATE.lock().unwrap();

        if index >= state.database.len() {
            return "Error: invalid file index".to_string();
        }

        state.database[index].ct.clone()
    };

    // -------- Step 3: decrypt --------
    let mut pt = Plaintext::new();

    kribe_core::decryption(&params, &sk, &mut ct, &mut pt);

    pt.format_full()
}