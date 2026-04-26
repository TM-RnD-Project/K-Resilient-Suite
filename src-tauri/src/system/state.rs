use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

use mcore::ed25519::big;
use mcore::ed25519::ecp;

use crate::kr_ibe::{params::Params as IbeParams, private_key::PrivateKey, ciphertext::Ciphertext};
use crate::kr_peks::{params::Params as PeksParams, public_key::PublicKey, private_key::PrivateKey as PeksSK, ciphertext::Ciphertext as PeksCiphertext};
use crate::kr_ibi::params::Params as IbiParams;

#[derive(Clone)]
pub struct StoredData {
    pub ct: Ciphertext,
    pub index: PeksCiphertext,
    pub owner: String,
}

pub struct LoginSession {
    pub g_r: (ecp::ECP, ecp::ECP),
    pub c1: big::BIG,
    pub c2: big::BIG,
    pub r: (big::BIG, big::BIG),
}


pub struct AppState {
    pub ibe_params: Option<IbeParams>,
    pub ibi_params: Option<IbiParams>,
    pub peks_params: Option<PeksParams>,

    pub users: HashMap<String, PrivateKey>,

    pub peks_pk: Option<PublicKey>,
    pub peks_sk: Option<PeksSK>,

    pub database: Vec<StoredData>,

    pub login_sessions: HashMap<String, LoginSession>,
    pub active_sessions: HashMap<String, bool>,
}

pub static APP_STATE: Lazy<Mutex<AppState>> = Lazy::new(|| {
    Mutex::new(AppState {
        ibe_params: None,
        ibi_params: None,
        peks_params: None,
        users: HashMap::new(),
        peks_pk: None,
        peks_sk: None,
        database: Vec::new(),
        login_sessions: HashMap::new(),
        active_sessions: HashMap::new(),
    })
});