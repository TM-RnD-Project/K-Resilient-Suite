use crate::system::state::APP_STATE;
use crate::kr_ibe::{main as kribe_core, private_key::PrivateKey};

pub fn register_user(id: &str) {
    let mut state = APP_STATE.lock().unwrap();

    let params = state.ibe_params.as_ref().unwrap();

    let mut sk = PrivateKey::new();

    kribe_core::extract(params, &mut sk, &id.as_bytes().to_vec());

    state.users.insert(id.to_string(), sk);
}