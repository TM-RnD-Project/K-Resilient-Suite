use crate::system::state::{APP_STATE, StoredData};

use crate::kr_ibe::{main as kribe_core, ciphertext::Ciphertext};
use crate::kr_peks::main as krpeks_core;

pub fn upload(sender: &str, receiver: &str, msg: &str, keyword: &str) {
    let mut state = APP_STATE.lock().unwrap();

    let ibe_params = state.ibe_params.as_ref().unwrap();
    let peks_params = state.peks_params.as_ref().unwrap();
    let pk = state.peks_pk.as_ref().unwrap();

    // -------- IBE --------
    let mut ct = Ciphertext::new();

    let receiver_bytes = receiver.as_bytes().to_vec();
    let msg_bytes = msg.as_bytes().to_vec();

    kribe_core::encryption(
        ibe_params,
        &mut ct,
        &receiver_bytes,
        &msg_bytes,
    );

    // -------- PEKS --------
    let index = krpeks_core::peks(peks_params, pk, &keyword.as_bytes().to_vec())
        .expect("PEKS failed");

    state.database.push(StoredData {
        ct,
        index,
        owner: receiver.to_string(),
    });
}