use crate::config::PREBUILT_TARGET_BYTES;
use base116::decode;

pub fn get_prebuilt() -> Vec<String> {
    let encoded_vec_u8: Vec<u8> = PREBUILT_TARGET_BYTES.into();
    let decoded_vec_u8: Vec<u8> = decode::decode_bytes(encoded_vec_u8)
        .into_iter()
        .map(|c| c.unwrap())
        .collect();
    let decoded_string = String::from_utf8_lossy(&decoded_vec_u8);
    let decoded_vec_string: Vec<String> = decoded_string.split("N").map(str::to_string).collect();
    decoded_vec_string
}
