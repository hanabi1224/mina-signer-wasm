use crate::*;
use mina_serialization_types::{json::*, signatures::*};
use mina_signer::CompressedPubKey;
use std::fmt::Display;
use wasm_bindgen::JsError;

pub(crate) fn map_js_err<T: Display>(err: T) -> JsError {
    JsError::new(&format!("{err}"))
}

pub(crate) fn compressed_pubkey_to_json(v: CompressedPubKey) -> PublicKeyJson {
    let ccp = CompressedCurvePoint {
        // This unwrap of a slice conversion is safe as a CompressedPubKey always has 32 bytes of data which the exact length of
        // FieldElement
        x: v.x
            .to_bytes()
            .as_slice()
            .try_into()
            .expect("Wrong number of bytes encountered when converting to FieldElement"),
        is_odd: v.is_odd,
    };
    ccp.into()
}

pub(crate) fn signature_to_json(t: MinaSignature) -> SignatureJson {
    let v1 = SignatureV1(
        (
            // This unwrap of a slice conversion is safe as a CompressedPubKey always has 32 bytes of data which the exact length of
            // FieldElement
            t.rx.to_bytes()
                .as_slice()
                .try_into()
                .expect("Wrong number of bytes encountered when converting to FieldElement"),
            t.s.to_bytes()
                .as_slice()
                .try_into()
                .expect("Wrong number of bytes encountered when converting to FieldElement"),
        )
            .into(),
    );
    v1.into()
}

pub(crate) fn string_to_memo(s: Option<String>) -> [u8; constants::MEMO_BYTES] {
    let mut memo = [0; constants::MEMO_BYTES];
    memo[0] = 1;
    if let Some(s) = s {
        for (i, &b) in s.as_bytes().iter().enumerate() {
            memo[i] = b;
        }
    }
    memo
}

#[wasm_bindgen(inline_js = r#"
        function js_to_string(v) {
            return `${v}`
        }

        module.exports = {
            js_to_string
        }
    "#)]
extern "C" {
    pub(crate) fn js_to_string(v: &JsValue) -> String;
}