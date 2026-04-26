use crate::system::state::APP_STATE;

use crate::kr_ibe::main as kribe_core;
use crate::kr_peks::main as krpeks_core;
use crate::kr_ibi::main as kribi_core;

use crate::kr_ibe::params::Params as IbeParams;
use crate::kr_peks::{params::Params as PeksParams, public_key::PublicKey, private_key::PrivateKey as PeksSK};
use crate::kr_ibi::params::Params as IbiParams;

pub fn setup_all(k: usize) {
    let mut state = APP_STATE.lock().unwrap();

    // -------- KR-IBE --------
    let mut ibe_params = IbeParams::new();
    kribe_core::setup(&mut ibe_params, k);
    state.ibe_params = Some(ibe_params);

    // -------- KR-IBI --------
    let mut ibi_params = IbiParams::new();
    kribi_core::setup(&mut ibi_params, k);
    state.ibi_params = Some(ibi_params);

    // -------- KR-PEKS --------
    let mut peks_params = PeksParams::new();
    krpeks_core::setup(&mut peks_params, k);

    let mut pk = PublicKey::new();
    let mut sk = PeksSK::new();

    krpeks_core::keygen(&peks_params, &mut pk, &mut sk);

    state.peks_params = Some(peks_params);
    state.peks_pk = Some(pk);
    state.peks_sk = Some(sk);
}