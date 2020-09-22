use serde_cose::{self, Curve, KeyType};
use std::{env, fs::read_to_string, path::PathBuf};
lazy_static! {
    static ref EXAMPLES_DIR: String = format!(
        "{}/../../../tests/fixtures/Examples",
        PathBuf::from(env::current_exe().unwrap())
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
    );
}
#[derive(Deserialize, Debug)]
struct Test {
    input: Input,
    output: Output,
}
#[derive(Deserialize, Debug)]
struct Input {
    plaintext: String,
    sign0: Sign0,
}
#[derive(Deserialize, Debug)]
struct Sign0 {
    key: Key,
}
#[derive(Deserialize, Debug)]
struct Key {
    kty: String,
    kid: String,
    crv: String,
    x_hex: Option<String>,
    y_hex: Option<String>,
    d_hex: Option<String>,
}

impl From<Key> for serde_cose::Key {
    fn from(key: Key) -> Self {
        serde_cose::Key {
            kty: match key.kty.as_str() {
                "OKP" => KeyType::OKP,
                _ => panic!("Key type found"),
            },
            crv: match key.crv.as_str() {
                "Ed25519" => Curve::ED25519,
                _ => panic!("Curve not found"),
            },
            kid: key.kid.as_bytes().to_vec(),
            d: key.d_hex.map(|d_hex| hex::decode(d_hex).unwrap()),
            x: key.x_hex.map(|x_hex| hex::decode(x_hex).unwrap()),
            y: key.y_hex.map(|y_hex| hex::decode(y_hex).unwrap()),
        }
    }
}

#[derive(Deserialize, Debug)]
struct Output {
    cbor: String,
}
#[test]
fn deserializes_sign1_ed25519() -> Result<(), std::io::Error> {
    let test: Test = serde_json::from_str(&read_to_string(format!(
        "{}/eddsa-examples/eddsa-sig-01.json",
        *EXAMPLES_DIR
    ))?)?;

    let cose = serde_cose::from_slice(&hex::decode(test.output.cbor).unwrap());
    let key: serde_cose::Key = test.input.sign0.key.into();
    assert!(key.verify(&cose));
    Ok(())
}
