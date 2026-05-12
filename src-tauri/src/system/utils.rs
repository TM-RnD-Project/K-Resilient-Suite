pub fn id_to_bytes(id: &str) -> Vec<u8> {
    let mut bytes = id.as_bytes().to_vec();

    if bytes.len() < 32 {
        bytes.resize(32, 0);
    }

    bytes
}