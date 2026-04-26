use crate::system::state::APP_STATE;
use crate::system::state::LoginSession;
use crate::kr_ibi::main as kribi_core;
use crate::kr_peks::{self, utils::*};

pub fn login_start(id: &str) -> (String, String) {
    let mut state = APP_STATE.lock().unwrap();
    let params = state.ibi_params.as_ref().unwrap();

    let mut rng = kribi_core::gen_seed();

    let (g_r, r) = kribi_core::commit(params, &mut rng);
    let (c1, c2) = kribi_core::challenge(params, &mut rng);

    state.login_sessions.insert(id.to_string(), LoginSession {
        g_r,
        c1,
        c2,
        r,
    });

    (
        kribi_core::big_to_hex(&c1),
        kribi_core::big_to_hex(&c2),
    )
}

pub fn login_respond(id: &str) -> (String, String) {
    let mut state = APP_STATE.lock().unwrap();

    let params = state.ibi_params.as_ref().unwrap();
    let session = state.login_sessions.get(id).unwrap();

    let id_bytes = id.as_bytes().to_vec();
    let (f1, f2) = kribi_core::extract(params, &id_bytes);

    let (s1, s2) = kribi_core::respond(
        &session.r,
        &(session.c1, session.c2),
        &(f1, f2),
        params.get_order(),
    );

    (
        kribi_core::big_to_hex(&s1),
        kribi_core::big_to_hex(&s2),
    )
}

pub fn login_verify(id: &str, s1_hex: &str, s2_hex: &str) -> bool {
    let mut state = APP_STATE.lock().unwrap();

    let params = state.ibi_params.as_ref().unwrap();
    let session = state.login_sessions.get(id).unwrap();

    let s1 = kribi_core::hex_to_big(s1_hex);
    let s2 = kribi_core::hex_to_big(s2_hex);

    let valid = kribi_core::verify(
        params,
        &session.g_r,
        &(s1, s2),
        &(session.c1, session.c2),
        &id.as_bytes().to_vec(),
    );

    if valid {
        state.active_sessions.insert(id.to_string(), true);
    }

    valid
}