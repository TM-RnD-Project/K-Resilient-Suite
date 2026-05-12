use crate::system::state::APP_STATE;
use crate::kr_ibe::{main as kribe_core, private_key::PrivateKey};
use crate::system::utils::id_to_bytes;

pub fn register_user(id: &str) {
    let mut state = APP_STATE.lock().unwrap();

    if state.ibe_params.is_none() {
        panic!("IBE params not initialised. Call setup_all() first.");
    }

    let params = state.ibe_params.as_ref().unwrap();

    let mut sk = PrivateKey::new();

    let mut id_bytes = id_to_bytes(id);

    kribe_core::extract(params, &mut sk, &id_bytes);
    println!("Register called for ID: {}", id);

    state.users.insert(id.to_string(), sk);
}