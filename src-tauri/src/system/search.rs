use crate::system::state::APP_STATE;
use crate::kr_peks::main as krpeks_core;

pub fn search(user: &str, keyword: &str) -> Vec<usize> {
    let state = APP_STATE.lock().unwrap();

    let peks_params = state.peks_params.as_ref().unwrap();
    let sk = state.peks_sk.as_ref().unwrap();

    let trapdoor = krpeks_core::trapdoor(peks_params, sk, &keyword.as_bytes().to_vec());

    state.database
        .iter()
        .enumerate()
        .filter(|(_, data)| {
            krpeks_core::test(&data.index, &trapdoor)
        })
        .map(|(i, _)| i)
        .collect()
}